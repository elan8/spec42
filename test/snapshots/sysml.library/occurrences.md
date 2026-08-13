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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/occurrences.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 22 39) (end 22 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 38 40) (end 38 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 56) (end 39 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 49 2) (end 49 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 23) (end 51 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 40) (end 51 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 65 41) (end 65 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 73 51) (end 73 92))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 87 3) (end 87 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 104 52) (end 104 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 106 3) (end 106 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 106 10) (end 106 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 106 37) (end 106 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 106 45) (end 106 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 124 3) (end 124 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 124 12) (end 124 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 124 41) (end 124 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 124 46) (end 124 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 141 51) (end 141 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 143 3) (end 143 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 143 10) (end 143 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 143 40) (end 143 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 143 48) (end 143 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 144 3) (end 144 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 144 10) (end 144 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 144 38) (end 144 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 144 46) (end 144 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 147 3) (end 147 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 147 10) (end 147 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 147 52) (end 147 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 147 60) (end 147 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 157 49) (end 157 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 158 49) (end 158 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 161 3) (end 163 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 166 3) (end 166 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 166 10) (end 166 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 167 4) (end 167 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 167 12) (end 167 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 177 46) (end 177 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 181 3) (end 181 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 181 10) (end 181 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 181 48) (end 181 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 181 56) (end 181 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 3) (end 184 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 10) (end 184 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 44) (end 184 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 184 52) (end 184 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 196 3) (end 196 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 196 10) (end 196 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 196 51) (end 196 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 196 59) (end 196 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 205 3) (end 205 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 205 13) (end 205 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 206 3) (end 206 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 218 21) (end 218 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 218 44) (end 218 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 219 21) (end 219 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 219 44) (end 219 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 222 3) (end 224 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 227 3) (end 227 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 227 10) (end 227 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 228 4) (end 228 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 228 12) (end 228 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 244 51) (end 244 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 247 3) (end 249 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 258 49) (end 258 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 262 4) (end 262 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 267 32) (end 267 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 275 2) (end 275 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 277 32) (end 277 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 287 2) (end 288 77))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 297 46) (end 297 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 323 54) (end 323 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 327 3) (end 327 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 327 10) (end 327 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 327 43) (end 327 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 327 51) (end 327 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 336 3) (end 336 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 338 2) (end 338 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 361 2) (end 361 123))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 363 2) (end 370 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 380 52) (end 380 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 381 6) (end 381 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 381 13) (end 381 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 381 26) (end 381 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 384 3) (end 391 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 409 55) (end 409 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 411 3) (end 411 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 414 3) (end 414 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 414 10) (end 414 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 414 45) (end 414 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 414 53) (end 414 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 430 54) (end 430 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 432 3) (end 432 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 435 3) (end 435 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 435 10) (end 435 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 435 45) (end 435 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 435 53) (end 435 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 438 20) (end 438 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 447 21) (end 447 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 450 3) (end 452 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 453 3) (end 455 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 457 2) (end 457 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 459 27) (end 459 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 468 21) (end 468 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 473 3) (end 475 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 476 3) (end 478 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 479 3) (end 481 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 483 2) (end 483 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 485 25) (end 485 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 493 21) (end 493 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 495 46) (end 495 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 495 56) (end 495 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 496 45) (end 496 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 497 22) (end 497 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 498 22) (end 498 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 499 57) (end 499 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 502 3) (end 504 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 506 3) (end 506 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 507 3) (end 507 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 509 2) (end 509 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 528 2) (end 528 98))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 539 3) (end 539 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 553 3) (end 553 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 554 3) (end 555 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 565 3) (end 565 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 568 2) (end 568 107))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 569 2) (end 569 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 571 2) (end 573 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 575 2) (end 577 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 588 45) (end 588 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 591 3) (end 591 115))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 592 3) (end 592 95))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 593 3) (end 593 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 594 3) (end 594 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 603 51) (end 603 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 606 3) (end 608 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 611 21) (end 611 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 617 2) (end 617 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 619 33) (end 619 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 619 67) (end 619 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 625 25) (end 625 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 626 25) (end 626 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 629 23) (end 629 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 636 3) (end 636 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 638 30) (end 638 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 646 2) (end 646 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 662 25) (end 662 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 663 25) (end 663 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 663 34) (end 663 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 666 33) (end 666 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 666 67) (end 666 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 672 25) (end 672 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 673 25) (end 673 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 682 25) (end 682 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 682 34) (end 682 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 683 25) (end 683 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 688 2) (end 693 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 696 66) (end 696 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 698 44) (end 698 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 699 9) (end 699 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 700 9) (end 700 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 701 18) (end 701 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 705 19) (end 705 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 705 28) (end 705 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 705 51) (end 705 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 708 40) (end 708 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 716 2) (end 716 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 717 2) (end 717 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 719 59) (end 719 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 720 59) (end 720 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 720 73) (end 720 109))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 721 2) (end 721 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 723 32) (end 723 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 723 57) (end 723 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 724 32) (end 724 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 724 57) (end 724 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 725 2) (end 725 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 730 31) (end 730 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 740 53) (end 740 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 741 53) (end 741 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 755 2) (end 755 91))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 766 2) (end 766 122))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 769 29) (end 769 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 779 59) (end 779 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 780 59) (end 780 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 792 49) (end 792 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 793 2) (end 793 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 807 2) (end 807 97))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 820 2) (end 820 136))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 830 2) (end 830 105))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 840 2) (end 840 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 850 2) (end 850 113))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 863 2) (end 863 116))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 874 2) (end 874 118))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 877 31) (end 877 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 885 58) (end 885 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 887 55) (end 887 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 920 71) (end 920 82))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 927 88) (end 927 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 928 84) (end 928 103))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:cf7bd88fac75f82be150fa81b0a1bfa6503cd2085f09103acdeb285bb41fe108") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::things") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::DataValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Links") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Clocks") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::Set") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::OrderedSet") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "CollectionFunctions::contains") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::isEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::includes") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 13))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::union") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HappensLink")) (specialization (reference "Without"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "sourceOccurrence")) (redefinition (reference "separateOccurrenceToo"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "targetOccurrence")) (redefinition (reference "separateOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HappensLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "sourceOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensJustBefore"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HappensBefore"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "earlierOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "laterOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BinaryLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HappensDuring"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "shorterOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Performances::BooleanEvaluation"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfers::Transfer") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t1First"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfers::Transfer") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OutsideOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpace"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpaceToo"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpaceLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OutsideOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpaceToo"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpace"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "JustOutsideOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpace"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpaceToo"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Anything"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OrderedSet"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::difference"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Set"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "elements")) (subsetting (reference "interdiff::elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "snapshots"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "subset")) (expressionOperand (reference "superendshot")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "self::timeCoincidentOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "predecessors"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "successors")) (expressionOperand (reference "disjoint")) (expressionOperand (reference "from")) (memberAccessOperand (reference "earlierOccurrence::successors")) (memberAccessOperand (reference "laterOccurrence::predecessors"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IncomingTransferSort")) (expressionOperand (reference "earlierFirstIncomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfers::Transfer")) (subsetting (reference "Transfers::transfers"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfersToSelf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "incomingTransfers"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target")) (expressionOperand (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "outsideOfOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "innerSpaceOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::hOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Set"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceTimeEnclosedPoints"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::intersection"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "outsideOfOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clock")) (expressionOperand (reference "universalClock"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "justOutsideOfOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "spaceBoundary"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "spaceInterior"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeSlices"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outerSpaceDimension"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transfers::Transfer")) (subsetting (reference "Transfers::transfers"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfersFromSelf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "outgoingTransfers"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "source")) (expressionOperand (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "withoutOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Life")) (subsetting (reference "portionOf")) (expressionOperand (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceTimeEnclosedOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "portionOfLife"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "withoutOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "things"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeSlices")) (subsetting (reference "spaceSlices")) (subsetting (reference "spaceTimeCoincidentOccurrences")) (subsetting (reference "sameLifeOccurrences")) (redefinition (reference "Anything::self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeSliceOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeSlices"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceShots"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceSlices"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isClosed"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "innerSpaceDimension")) (memberAccessOperand (reference "spaceBounder::innerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceSlices"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isClosed"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "innerSpaceDimension")) (memberAccessOperand (reference "spaceBounder::innerSpaceDimension"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceShotOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "occurrences")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "smallerSpace::spaceEnclosedOccurrences")) (memberAccessOperand (reference "largerSpace::spaceEnclosedOccurrences")) (memberAccessOperand (reference "smallerSpace::outsideOfOccurrences")) (memberAccessOperand (reference "largerSpace::outsideOfOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceSlices"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceSliceOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceSliceOf")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "spaceShottedOccurrence::spaceShotOf")) (memberAccessOperand (reference "spaceShotOccurrence::spaceShotOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceSlices"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "portionOf")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "spaceSlicedOccurrence::spaceSliceOf")) (memberAccessOperand (reference "spaceSliceOccurrence::spaceSliceOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "portions"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeCoincidentOccurrences")) (subsetting (reference "spaceEnclosedOccurrences")) (subsetting (reference "spaceTimeEnclosedOccurrences")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "thatOccurrence::spaceTimeCoincidentOccurrences")) (memberAccessOperand (reference "thisOccurrence::spaceTimeCoincidentOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "largerSpace")) (redefinition (reference "thatOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "smallerSpace")) (redefinition (reference "thisOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeEnclosedOccurrences")) (subsetting (reference "spaceEnclosedOccurrences")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "largerSpace::spaceTimeEnclosedOccurrences")) (memberAccessOperand (reference "smallerSpace::spaceTimeEnclosedOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "spaceTimeEnclosedOccurrences")) (expressionOperand (reference "redefines"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "snapshots"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "occurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "withoutOccurrences")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "laterOccurrence::successors")) (memberAccessOperand (reference "earlierOccurrence::successors"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "occurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "outsideOfOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "timeEnclosedOccurrences")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "thatOccurrence::timeCoincidentOccurrences")) (memberAccessOperand (reference "thisOccurrence::timeCoincidentOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "longerOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "shorterOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "occurrences")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "longerOccurrence::predecessors")) (memberAccessOperand (reference "shorterOccurrence::predecessors")) (memberAccessOperand (reference "longerOccurrence::successors")) (memberAccessOperand (reference "shorterOccurrence::successors")) (memberAccessOperand (reference "shorterOccurrence::timeEnclosedOccurrences")) (memberAccessOperand (reference "longerOccurrence::timeEnclosedOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "portionOf")) (expressionOperand (reference "subset")) (expressionOperand (reference "subsets")) (memberAccessOperand (reference "timeSlicedOccurrence::timeSliceOf")) (memberAccessOperand (reference "timeSliceOccurrence::timeSliceOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "that"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "portions"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Set"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf::union"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpaceLink")) (specialization (reference "Without"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "targetOccurrence")) (redefinition (reference "separateOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "sourceOccurrence")) (redefinition (reference "separateOccurrenceToo"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Within"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "smallerOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfLink"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SelfSameLifeLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BinaryLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceDataValue"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DataValue")) (subsetting (reference "myselfSameLife"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "myselfSameLife"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetDataValue"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DataValue")) (subsetting (reference "selfSameLife"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "selfSameLife")) (subsetting (reference "sourceOccurrence::sameLifeOccurrences"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TimeSliceOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "timeSliceOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BinaryLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SpaceSliceOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "spaceSliceOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PortionOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "portionOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OutsideOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpaceToo"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "separateSpace"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PortionOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "portionOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "HappensDuring")) (specialization (reference "InsideOf"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "shorterOccurrence")) (redefinition (reference "smallerSpace"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Within")) (specialization (reference "HappensWhile"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "smallerOccurrence")) (redefinition (reference "HappensWhile::thisOccurrence"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BinaryLink"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "BinaryLink::source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IncomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "t1::endShot::successors")) (memberAccessOperand (reference "t2::endShot")) (invocationCallee (reference "includes"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensBefore")) (subsetting (reference "binaryLinks"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "HappensBefore::earlierOccurrence")) (redefinition (reference "binaryLinks::source"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (redefinition (reference "HappensBefore::laterOccurrence")) (redefinition (reference "binaryLinks::target"))))
    (declaration (id (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (subsetting (reference "things"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Links")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Clocks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::things")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::Set")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::OrderedSet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "CollectionFunctions::contains")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::union")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind specialization) (ordinal 0))
      (authored-target "HappensLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind specialization) (ordinal 1))
      (authored-target "Without")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "sourceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind redefinition) (ordinal 1))
      (authored-target "separateOccurrenceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "targetOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind redefinition) (ordinal 1))
      (authored-target "separateOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (kind specialization) (ordinal 0))
      (authored-target "HappensLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "sourceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensJustBefore"))) (kind specialization) (ordinal 0))
      (authored-target "HappensBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "earlierOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "laterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink"))) (kind specialization) (ordinal 0))
      (authored-target "BinaryLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile"))) (kind specialization) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "shorterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort"))) (kind specialization) (ordinal 0))
      (authored-target "Performances::BooleanEvaluation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t1First"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf"))) (kind specialization) (ordinal 0))
      (authored-target "OutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpaceLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf"))) (kind specialization) (ordinal 0))
      (authored-target "OutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life"))) (kind specialization) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith"))) (kind specialization) (ordinal 0))
      (authored-target "JustOutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (kind specialization) (ordinal 0))
      (authored-target "Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "OrderedSet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::difference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff"))) (kind featureTyping) (ordinal 0))
      (authored-target "Set")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (kind subsetting) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind subsetting) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind subsetting) (ordinal 1))
      (authored-target "interdiff::elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (kind subsetting) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind expressionOperand) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind subsetting) (ordinal 0))
      (authored-target "snapshots")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind expressionOperand) (ordinal 1))
      (authored-target "superendshot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind expressionOperand) (ordinal 2))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "self::timeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind subsetting) (ordinal 0))
      (authored-target "predecessors")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind subsetting) (ordinal 0))
      (authored-target "successors")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind expressionOperand) (ordinal 0))
      (authored-target "disjoint")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind expressionOperand) (ordinal 1))
      (authored-target "from")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "earlierOccurrence::successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "laterOccurrence::predecessors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind featureTyping) (ordinal 0))
      (authored-target "IncomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind expressionOperand) (ordinal 0))
      (authored-target "earlierFirstIncomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers"))) (kind subsetting) (ordinal 0))
      (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfersToSelf"))) (kind subsetting) (ordinal 0))
      (authored-target "incomingTransfers")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "outsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "innerSpaceOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::hOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Set")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceTimeEnclosedPoints")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::intersection"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "outsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (kind expressionOperand) (ordinal 0))
      (authored-target "universalClock")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "justOutsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "spaceBoundary")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "spaceInterior")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outerSpaceDimension"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers"))) (kind subsetting) (ordinal 0))
      (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfersFromSelf"))) (kind subsetting) (ordinal 0))
      (authored-target "outgoingTransfers")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "withoutOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind featureTyping) (ordinal 0))
      (authored-target "Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind subsetting) (ordinal 0))
      (authored-target "portionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind expressionOperand) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceTimeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "portionOfLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind subsetting) (ordinal 0))
      (authored-target "withoutOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind expressionOperand) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "things")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 1))
      (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 2))
      (authored-target "spaceTimeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 3))
      (authored-target "sameLifeOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Anything::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceShots")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isClosed")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "spaceBounder::innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isClosed")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "spaceBounder::innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceShotOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 2))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 3))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "smallerSpace::spaceEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "largerSpace::spaceEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 2))
      (authored-target "smallerSpace::outsideOfOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 3))
      (authored-target "largerSpace::outsideOfOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "spaceShottedOccurrence::spaceShotOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "spaceShotOccurrence::spaceShotOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind subsetting) (ordinal 0))
      (authored-target "portionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "spaceSlicedOccurrence::spaceSliceOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "spaceSliceOccurrence::spaceSliceOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind subsetting) (ordinal 0))
      (authored-target "portions")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "timeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 1))
      (authored-target "spaceEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 2))
      (authored-target "spaceTimeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "thatOccurrence::spaceTimeCoincidentOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "thisOccurrence::spaceTimeCoincidentOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "largerSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "smallerSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "thatOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "thisOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "timeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind subsetting) (ordinal 1))
      (authored-target "spaceEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "largerSpace::spaceTimeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "smallerSpace::spaceTimeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind subsetting) (ordinal 0))
      (authored-target "spaceTimeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind expressionOperand) (ordinal 0))
      (authored-target "redefines")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind subsetting) (ordinal 0))
      (authored-target "snapshots")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind subsetting) (ordinal 0))
      (authored-target "withoutOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "laterOccurrence::successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "earlierOccurrence::successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "outsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind expressionOperand) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "timeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "thatOccurrence::timeCoincidentOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "thisOccurrence::timeCoincidentOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "longerOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "shorterOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 2))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 3))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 4))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 5))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "longerOccurrence::predecessors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "shorterOccurrence::predecessors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 2))
      (authored-target "longerOccurrence::successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 3))
      (authored-target "shorterOccurrence::successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 4))
      (authored-target "shorterOccurrence::timeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 5))
      (authored-target "longerOccurrence::timeEnclosedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind subsetting) (ordinal 0))
      (authored-target "portionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind expressionOperand) (ordinal 0))
      (authored-target "subset")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "timeSlicedOccurrence::timeSliceOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "timeSliceOccurrence::timeSliceOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind subsetting) (ordinal 0))
      (authored-target "portions")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Set")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf::union"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpaceLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind specialization) (ordinal 1))
      (authored-target "Without")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "targetOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind redefinition) (ordinal 1))
      (authored-target "separateOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind redefinition) (ordinal 0))
      (authored-target "sourceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind redefinition) (ordinal 1))
      (authored-target "separateOccurrenceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (kind specialization) (ordinal 0))
      (authored-target "Within")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "smallerOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfLink"))) (kind specialization) (ordinal 0))
      (authored-target "SelfSameLifeLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink"))) (kind specialization) (ordinal 0))
      (authored-target "BinaryLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceDataValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceDataValue"))) (kind subsetting) (ordinal 0))
      (authored-target "myselfSameLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "myselfSameLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetDataValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "DataValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetDataValue"))) (kind subsetting) (ordinal 0))
      (authored-target "selfSameLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind subsetting) (ordinal 0))
      (authored-target "selfSameLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind subsetting) (ordinal 1))
      (authored-target "sourceOccurrence::sameLifeOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf"))) (kind specialization) (ordinal 0))
      (authored-target "TimeSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "timeSliceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink"))) (kind specialization) (ordinal 0))
      (authored-target "BinaryLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf"))) (kind specialization) (ordinal 0))
      (authored-target "SpaceSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "spaceSliceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf"))) (kind specialization) (ordinal 0))
      (authored-target "PortionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "portionOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy"))) (kind specialization) (ordinal 0))
      (authored-target "OutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind redefinition) (ordinal 0))
      (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf"))) (kind specialization) (ordinal 0))
      (authored-target "PortionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "portionOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind specialization) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind specialization) (ordinal 1))
      (authored-target "InsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "shorterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind redefinition) (ordinal 1))
      (authored-target "smallerSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind specialization) (ordinal 0))
      (authored-target "Within")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind specialization) (ordinal 1))
      (authored-target "HappensWhile")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "smallerOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind redefinition) (ordinal 1))
      (authored-target "HappensWhile::thisOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without"))) (kind specialization) (ordinal 0))
      (authored-target "BinaryLink")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (kind redefinition) (ordinal 0))
      (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort"))) (kind featureTyping) (ordinal 0))
      (authored-target "IncomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "t1::endShot::successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "t2::endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind invocationCallee) (ordinal 0))
      (authored-target "includes")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (kind subsetting) (ordinal 0))
      (authored-target "binaryLinks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "HappensBefore::earlierOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind redefinition) (ordinal 1))
      (authored-target "binaryLinks::source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind redefinition) (ordinal 0))
      (authored-target "HappensBefore::laterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind redefinition) (ordinal 1))
      (authored-target "binaryLinks::target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (kind subsetting) (ordinal 0))
      (authored-target "things")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind redefinition) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind redefinition) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensJustBefore"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensJustBefore"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::difference"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::difference"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfersToSelf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfersToSelf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::hOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::hOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::intersection"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::intersection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfersFromSelf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfersFromSelf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind subsetting) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 2)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 3)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 2)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind subsetting) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf::union"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf::union"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind redefinition) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind redefinition) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfLink"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfLink"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind redefinition) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind redefinition) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/occurrences.md") (range (start 12 16) (end 12 24)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0) (authored-target "Links")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 13 16) (end 13 25)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0) (authored-target "Clocks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 8 16) (end 8 28)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Base::things")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 9 16) (end 9 31)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Base::DataValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 10 16) (end 10 37)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 11 16) (end 11 37)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 14 16) (end 14 32)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::Set")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 15 16) (end 15 39)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::OrderedSet")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 16 16) (end 16 45)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "CollectionFunctions::contains")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 17 16) (end 17 42)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 18 16) (end 18 43)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 19 16) (end 19 43)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::includes")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 20 16) (end 20 40)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind import) (ordinal 13))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::union")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 891 37) (end 891 48)) (probe (position 891 37))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind specialization) (ordinal 0) (authored-target "HappensLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 891 50) (end 891 57)) (probe (position 891 50))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore"))) (kind specialization) (ordinal 1) (authored-target "Without")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 903 33) (end 903 43)) (probe (position 903 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 903 54) (end 903 70)) (probe (position 903 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "sourceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 903 72) (end 903 93)) (probe (position 903 72))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence"))) (kind redefinition) (ordinal 1) (authored-target "separateOccurrenceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 905 31) (end 905 41)) (probe (position 905 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 905 52) (end 905 68)) (probe (position 905 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "targetOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 905 70) (end 905 88)) (probe (position 905 70))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence"))) (kind redefinition) (ordinal 1) (authored-target "separateOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 744 37) (end 744 48)) (probe (position 744 37))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring"))) (kind specialization) (ordinal 0) (authored-target "HappensLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 754 33) (end 754 43)) (probe (position 754 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 754 54) (end 754 70)) (probe (position 754 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "sourceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 909 41) (end 909 54)) (probe (position 909 41))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensJustBefore"))) (kind specialization) (ordinal 0) (authored-target "HappensBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 916 43) (end 916 53)) (probe (position 916 43))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 917 41) (end 917 51)) (probe (position 917 41))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 916 24) (end 916 41)) (probe (position 916 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "earlierOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 917 24) (end 917 39)) (probe (position 917 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "laterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 730 31) (end 730 41)) (probe (position 730 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink"))) (kind specialization) (ordinal 0) (authored-target "BinaryLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 740 32) (end 740 42)) (probe (position 740 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 740 53) (end 740 71)) (probe (position 740 53))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::sourceOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 741 32) (end 741 42)) (probe (position 741 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 741 53) (end 741 71)) (probe (position 741 53))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensLink::targetOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 758 36) (end 758 49)) (probe (position 758 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile"))) (kind specialization) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 765 30) (end 765 40)) (probe (position 765 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 765 51) (end 765 68)) (probe (position 765 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "shorterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 698 44) (end 698 75)) (probe (position 698 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort"))) (kind specialization) (ordinal 0) (authored-target "Performances::BooleanEvaluation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 699 9) (end 699 28)) (probe (position 699 9))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t1"))) (kind featureTyping) (ordinal 0) (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 701 18) (end 701 25)) (probe (position 701 18))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t1First"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 700 9) (end 700 28)) (probe (position 700 9))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort::t2"))) (kind featureTyping) (ordinal 0) (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 970 36) (end 970 45)) (probe (position 970 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf"))) (kind specialization) (ordinal 0) (authored-target "OutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 978 26) (end 978 36)) (probe (position 978 26))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 978 47) (end 978 60)) (probe (position 978 47))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::innerSpace"))) (kind redefinition) (ordinal 0) (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 977 26) (end 977 36)) (probe (position 977 26))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 977 47) (end 977 63)) (probe (position 977 47))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InnerSpaceOf::outerSpace"))) (kind redefinition) (ordinal 0) (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 783 32) (end 783 41)) (probe (position 783 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf"))) (kind specialization) (ordinal 0) (authored-target "SpaceLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 792 28) (end 792 38)) (probe (position 792 28))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 792 49) (end 792 55)) (probe (position 792 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace"))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 945 37) (end 945 46)) (probe (position 945 37))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf"))) (kind specialization) (ordinal 0) (authored-target "OutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 952 42) (end 952 52)) (probe (position 952 42))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 954 39) (end 954 49)) (probe (position 954 39))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 952 24) (end 952 40)) (probe (position 952 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 954 24) (end 954 37)) (probe (position 954 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 687 37) (end 687 47)) (probe (position 687 37))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life"))) (kind specialization) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 958 33) (end 958 46)) (probe (position 958 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith"))) (kind specialization) (ordinal 0) (authored-target "JustOutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::JustOutsideOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 966 27) (end 966 37)) (probe (position 966 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 966 48) (end 966 61)) (probe (position 966 48))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpace"))) (kind redefinition) (ordinal 0) (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 964 30) (end 964 40)) (probe (position 964 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 964 51) (end 964 67)) (probe (position 964 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::MatesWith::matingSpaceToo"))) (kind redefinition) (ordinal 0) (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 22 39) (end 22 47)) (probe (position 22 39))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (kind specialization) (ordinal 0) (authored-target "Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 485 25) (end 485 35)) (probe (position 485 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf"))) (kind featureTyping) (ordinal 0) (authored-target "OrderedSet")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 493 31) (end 493 41)) (probe (position 493 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 493 21) (end 493 29)) (probe (position 493 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 494 23) (end 494 33)) (probe (position 494 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::difference"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 497 22) (end 497 25)) (probe (position 497 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff"))) (kind featureTyping) (ordinal 0) (authored-target "Set")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 498 32) (end 498 42)) (probe (position 498 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 498 22) (end 498 30)) (probe (position 498 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 499 31) (end 499 41)) (probe (position 499 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 499 57) (end 499 65)) (probe (position 499 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend"))) (kind subsetting) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 495 20) (end 495 30)) (probe (position 495 20))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 495 46) (end 495 54)) (probe (position 495 46))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind subsetting) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 495 56) (end 495 74)) (probe (position 495 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::minuend"))) (kind subsetting) (ordinal 1) (authored-target "interdiff::elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 496 23) (end 496 33)) (probe (position 496 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 496 45) (end 496 53)) (probe (position 496 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::differencesOf::subtrahend"))) (kind subsetting) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 635 26) (end 635 36)) (probe (position 635 26))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 635 49) (end 635 53)) (probe (position 635 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (kind expressionOperand) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 372 27) (end 372 37)) (probe (position 372 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 372 49) (end 372 58)) (probe (position 372 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot"))) (kind subsetting) (ordinal 0) (authored-target "snapshots")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 379 24) (end 379 34)) (probe (position 379 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 381 6) (end 381 12)) (probe (position 381 6))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 381 13) (end 381 25)) (probe (position 381 13))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind expressionOperand) (ordinal 1) (authored-target "superendshot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 381 26) (end 381 33)) (probe (position 381 26))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind expressionOperand) (ordinal 2) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 381 34) (end 381 64)) (probe (position 381 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot"))) (kind memberAccessOperand) (ordinal 0) (authored-target "self::timeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 380 29) (end 380 39)) (probe (position 380 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 380 52) (end 380 56)) (probe (position 380 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::endShot::subendshot::superendshot"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 109 33) (end 109 43)) (probe (position 109 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 109 58) (end 109 70)) (probe (position 109 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediatePredecessors"))) (kind subsetting) (ordinal 0) (authored-target "predecessors")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 117 31) (end 117 41)) (probe (position 117 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 117 56) (end 117 66)) (probe (position 117 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind subsetting) (ordinal 0) (authored-target "successors")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 124 3) (end 124 11)) (probe (position 124 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind expressionOperand) (ordinal 0) (authored-target "disjoint")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 124 41) (end 124 45)) (probe (position 124 41))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind expressionOperand) (ordinal 1) (authored-target "from")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 124 12) (end 124 40)) (probe (position 124 12))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind memberAccessOperand) (ordinal 0) (authored-target "earlierOccurrence::successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 124 46) (end 124 74)) (probe (position 124 46))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::immediateSuccessors"))) (kind memberAccessOperand) (ordinal 1) (authored-target "laterOccurrence::predecessors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 648 34) (end 648 54)) (probe (position 648 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind featureTyping) (ordinal 0) (authored-target "IncomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 648 70) (end 648 102)) (probe (position 648 70))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (kind expressionOperand) (ordinal 0) (authored-target "earlierFirstIncomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 619 33) (end 619 52)) (probe (position 619 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers"))) (kind featureTyping) (ordinal 0) (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 619 67) (end 619 87)) (probe (position 619 67))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers"))) (kind subsetting) (ordinal 0) (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 625 25) (end 625 31)) (probe (position 625 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 626 25) (end 626 31)) (probe (position 626 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 656 50) (end 656 67)) (probe (position 656 50))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfersToSelf"))) (kind subsetting) (ordinal 0) (authored-target "incomingTransfers")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransfers")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 662 25) (end 662 31)) (probe (position 662 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 663 25) (end 663 31)) (probe (position 663 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 663 34) (end 663 38)) (probe (position 663 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 267 32) (end 267 39)) (probe (position 267 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 579 33) (end 579 43)) (probe (position 579 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 579 59) (end 579 79)) (probe (position 579 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "outsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 585 21) (end 585 42)) (probe (position 585 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "innerSpaceOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 590 24) (end 590 34)) (probe (position 590 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::hOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 589 23) (end 589 33)) (probe (position 589 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 589 45) (end 589 49)) (probe (position 589 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::innerSpace"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 588 23) (end 588 33)) (probe (position 588 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 588 45) (end 588 49)) (probe (position 588 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences::outerSpace"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 459 27) (end 459 30)) (probe (position 459 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf"))) (kind featureTyping) (ordinal 0) (authored-target "Set")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 468 31) (end 468 41)) (probe (position 468 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 468 21) (end 468 29)) (probe (position 468 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 469 33) (end 469 43)) (probe (position 469 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 469 58) (end 469 81)) (probe (position 469 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::::notIntersection"))) (kind subsetting) (ordinal 0) (authored-target "spaceTimeEnclosedPoints")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 471 25) (end 471 35)) (probe (position 471 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::intersectionsOf::intersection"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 611 21) (end 611 28)) (probe (position 611 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 629 23) (end 629 30)) (probe (position 629 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 638 30) (end 638 37)) (probe (position 638 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 238 36) (end 238 46)) (probe (position 238 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 238 61) (end 238 81)) (probe (position 238 61))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "outsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 245 26) (end 245 36)) (probe (position 245 26))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 245 48) (end 245 52)) (probe (position 245 48))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpace"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 244 29) (end 244 39)) (probe (position 244 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 244 51) (end 244 55)) (probe (position 244 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences::separateSpaceToo"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 51 23) (end 51 28)) (probe (position 51 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (kind featureTyping) (ordinal 0) (authored-target "Clock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 51 40) (end 51 54)) (probe (position 51 40))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (kind expressionOperand) (ordinal 0) (authored-target "universalClock")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 252 29) (end 252 39)) (probe (position 252 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 252 54) (end 252 78)) (probe (position 252 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "justOutsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 260 29) (end 260 39)) (probe (position 260 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 261 30) (end 261 43)) (probe (position 261 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "spaceBoundary")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 263 30) (end 263 43)) (probe (position 263 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "spaceInterior")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 259 24) (end 259 34)) (probe (position 259 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 259 46) (end 259 50)) (probe (position 259 46))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpace"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 258 27) (end 258 37)) (probe (position 258 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 258 49) (end 258 53)) (probe (position 258 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::matingOccurrences::matingSpaceToo"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 354 35) (end 354 45)) (probe (position 354 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 354 60) (end 354 70)) (probe (position 354 60))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::middleTimeSlice"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 277 32) (end 277 39)) (probe (position 277 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outerSpaceDimension"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 666 33) (end 666 52)) (probe (position 666 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers"))) (kind featureTyping) (ordinal 0) (authored-target "Transfers::Transfer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 666 67) (end 666 87)) (probe (position 666 67))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers"))) (kind subsetting) (ordinal 0) (authored-target "Transfers::transfers")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 672 25) (end 672 31)) (probe (position 672 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 673 25) (end 673 31)) (probe (position 673 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 676 52) (end 676 69)) (probe (position 676 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfersFromSelf"))) (kind subsetting) (ordinal 0) (authored-target "outgoingTransfers")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outgoingTransfers")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 682 25) (end 682 31)) (probe (position 682 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 683 25) (end 683 31)) (probe (position 683 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 682 34) (end 682 38)) (probe (position 682 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 231 32) (end 231 42)) (probe (position 231 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 231 57) (end 231 75)) (probe (position 231 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "withoutOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 300 22) (end 300 32)) (probe (position 300 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 36 25) (end 36 29)) (probe (position 36 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind featureTyping) (ordinal 0) (authored-target "Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Life")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 36 41) (end 36 50)) (probe (position 36 41))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind subsetting) (ordinal 0) (authored-target "portionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 36 59) (end 36 63)) (probe (position 36 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (kind expressionOperand) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 290 32) (end 290 42)) (probe (position 290 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 290 57) (end 290 85)) (probe (position 290 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))) (kind subsetting) (ordinal 0) (authored-target "spaceTimeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 297 30) (end 297 43)) (probe (position 297 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "portionOfLife")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 90 24) (end 90 34)) (probe (position 90 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 90 49) (end 90 67)) (probe (position 90 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::predecessors"))) (kind subsetting) (ordinal 0) (authored-target "withoutOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 645 32) (end 645 42)) (probe (position 645 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 645 55) (end 645 59)) (probe (position 645 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (kind expressionOperand) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 39 31) (end 39 41)) (probe (position 39 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 39 56) (end 39 62)) (probe (position 39 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "things")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 38 16) (end 38 26)) (probe (position 38 16))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 38 63) (end 38 73)) (probe (position 38 63))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 38 75) (end 38 86)) (probe (position 38 75))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 1) (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 38 88) (end 38 118)) (probe (position 38 88))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 2) (authored-target "spaceTimeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 38 120) (end 38 139)) (probe (position 38 120))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind subsetting) (ordinal 3) (authored-target "sameLifeOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 38 40) (end 38 54)) (probe (position 38 40))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self"))) (kind redefinition) (ordinal 0) (authored-target "Anything::self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 340 23) (end 340 33)) (probe (position 340 23))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 340 48) (end 340 59)) (probe (position 340 48))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshotOf"))) (kind subsetting) (ordinal 0) (authored-target "timeSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 330 33) (end 330 43)) (probe (position 330 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 330 58) (end 330 68)) (probe (position 330 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 530 33) (end 530 43)) (probe (position 530 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 530 58) (end 530 68)) (probe (position 530 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary"))) (kind subsetting) (ordinal 0) (authored-target "spaceShots")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 548 18) (end 548 28)) (probe (position 548 18))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 548 44) (end 548 55)) (probe (position 548 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::inner"))) (kind subsetting) (ordinal 0) (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 549 22) (end 549 30)) (probe (position 549 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isClosed")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 550 22) (end 550 41)) (probe (position 550 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 550 44) (end 550 76)) (probe (position 550 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "spaceBounder::innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 543 18) (end 543 28)) (probe (position 543 18))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 543 44) (end 543 55)) (probe (position 543 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::outer"))) (kind subsetting) (ordinal 0) (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 544 22) (end 544 30)) (probe (position 544 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isClosed")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::isClosed")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 545 22) (end 545 41)) (probe (position 545 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 545 44) (end 545 76)) (probe (position 545 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "spaceBounder::innerSpaceDimension")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::innerSpaceDimension")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 541 25) (end 541 35)) (probe (position 541 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 541 48) (end 541 52)) (probe (position 541 48))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundary::spaceBounder"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 558 27) (end 558 37)) (probe (position 558 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 558 52) (end 558 63)) (probe (position 558 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf"))) (kind subsetting) (ordinal 0) (authored-target "spaceShotOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 564 27) (end 564 37)) (probe (position 564 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 564 46) (end 564 50)) (probe (position 564 46))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf::spaceBounderOf"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 170 36) (end 170 46)) (probe (position 170 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 170 61) (end 170 72)) (probe (position 170 61))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 181 3) (end 181 9)) (probe (position 181 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 181 48) (end 181 55)) (probe (position 181 48))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 184 3) (end 184 9)) (probe (position 184 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 2) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 184 44) (end 184 51)) (probe (position 184 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind expressionOperand) (ordinal 3) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 181 10) (end 181 47)) (probe (position 181 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 0) (authored-target "smallerSpace::spaceEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 181 56) (end 181 92)) (probe (position 181 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 1) (authored-target "largerSpace::spaceEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 184 10) (end 184 43)) (probe (position 184 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 2) (authored-target "smallerSpace::outsideOfOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 184 52) (end 184 84)) (probe (position 184 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 3) (authored-target "largerSpace::outsideOfOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 177 24) (end 177 34)) (probe (position 177 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 177 46) (end 177 50)) (probe (position 177 46))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 178 25) (end 178 35)) (probe (position 178 25))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 178 47) (end 178 51)) (probe (position 178 47))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 511 33) (end 511 43)) (probe (position 511 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 511 58) (end 511 69)) (probe (position 511 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInterior"))) (kind subsetting) (ordinal 0) (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 521 27) (end 521 37)) (probe (position 521 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 521 52) (end 521 64)) (probe (position 521 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceInteriorOf"))) (kind subsetting) (ordinal 0) (authored-target "spaceSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 424 27) (end 424 37)) (probe (position 424 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 424 52) (end 424 64)) (probe (position 424 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind subsetting) (ordinal 0) (authored-target "spaceSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 435 3) (end 435 9)) (probe (position 435 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 435 45) (end 435 52)) (probe (position 435 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 435 10) (end 435 44)) (probe (position 435 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind memberAccessOperand) (ordinal 0) (authored-target "spaceShottedOccurrence::spaceShotOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 435 53) (end 435 84)) (probe (position 435 53))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf"))) (kind memberAccessOperand) (ordinal 1) (authored-target "spaceShotOccurrence::spaceShotOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 430 32) (end 430 42)) (probe (position 430 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 430 54) (end 430 58)) (probe (position 430 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShotOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 431 35) (end 431 45)) (probe (position 431 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 431 57) (end 431 61)) (probe (position 431 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShotOf::spaceShottedOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 417 30) (end 417 40)) (probe (position 417 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 417 55) (end 417 66)) (probe (position 417 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceShots"))) (kind subsetting) (ordinal 0) (authored-target "spaceSlices")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 402 24) (end 402 34)) (probe (position 402 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 402 49) (end 402 58)) (probe (position 402 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind subsetting) (ordinal 0) (authored-target "portionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 414 3) (end 414 9)) (probe (position 414 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 414 45) (end 414 52)) (probe (position 414 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 414 10) (end 414 44)) (probe (position 414 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind memberAccessOperand) (ordinal 0) (authored-target "spaceSlicedOccurrence::spaceSliceOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 414 53) (end 414 86)) (probe (position 414 53))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf"))) (kind memberAccessOperand) (ordinal 1) (authored-target "spaceSliceOccurrence::spaceSliceOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 409 33) (end 409 43)) (probe (position 409 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 409 55) (end 409 59)) (probe (position 409 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSliceOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 410 34) (end 410 44)) (probe (position 410 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 410 56) (end 410 60)) (probe (position 410 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSliceOf::spaceSlicedOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 393 31) (end 393 41)) (probe (position 393 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 393 56) (end 393 64)) (probe (position 393 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceSlices"))) (kind subsetting) (ordinal 0) (authored-target "portions")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 209 42) (end 209 52)) (probe (position 209 42))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 210 11) (end 210 36)) (probe (position 210 11))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "timeCoincidentOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 210 38) (end 210 62)) (probe (position 210 38))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 1) (authored-target "spaceEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 210 64) (end 210 92)) (probe (position 210 64))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind subsetting) (ordinal 2) (authored-target "spaceTimeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 227 3) (end 227 9)) (probe (position 227 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 228 4) (end 228 11)) (probe (position 228 4))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 227 10) (end 227 55)) (probe (position 227 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 0) (authored-target "thatOccurrence::spaceTimeCoincidentOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 228 12) (end 228 57)) (probe (position 228 12))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 1) (authored-target "thisOccurrence::spaceTimeCoincidentOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 218 44) (end 218 55)) (probe (position 218 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "largerSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 219 44) (end 219 56)) (probe (position 219 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "smallerSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 218 21) (end 218 35)) (probe (position 218 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "thatOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 219 21) (end 219 35)) (probe (position 219 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "thisOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 187 44) (end 187 54)) (probe (position 187 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 187 69) (end 187 92)) (probe (position 187 69))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "timeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 187 94) (end 187 118)) (probe (position 187 94))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind subsetting) (ordinal 1) (authored-target "spaceEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 196 3) (end 196 9)) (probe (position 196 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 196 51) (end 196 58)) (probe (position 196 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 196 10) (end 196 50)) (probe (position 196 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 0) (authored-target "largerSpace::spaceTimeEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 196 59) (end 196 100)) (probe (position 196 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 1) (authored-target "smallerSpace::spaceTimeEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 199 40) (end 199 50)) (probe (position 199 40))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 199 65) (end 199 93)) (probe (position 199 65))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind subsetting) (ordinal 0) (authored-target "spaceTimeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 205 3) (end 205 12)) (probe (position 205 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints"))) (kind expressionOperand) (ordinal 0) (authored-target "redefines")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 347 29) (end 347 39)) (probe (position 347 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 347 51) (end 347 60)) (probe (position 347 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::startShot"))) (kind subsetting) (ordinal 0) (authored-target "snapshots")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::snapshots")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 59 36) (end 59 46)) (probe (position 59 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 59 61) (end 59 72)) (probe (position 59 61))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (kind subsetting) (ordinal 0) (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 65 22) (end 65 32)) (probe (position 65 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 73 22) (end 73 42)) (probe (position 73 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 97 22) (end 97 32)) (probe (position 97 22))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 97 47) (end 97 65)) (probe (position 97 47))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind subsetting) (ordinal 0) (authored-target "withoutOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 106 3) (end 106 9)) (probe (position 106 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 106 37) (end 106 44)) (probe (position 106 37))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 106 10) (end 106 36)) (probe (position 106 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind memberAccessOperand) (ordinal 0) (authored-target "laterOccurrence::successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 106 45) (end 106 73)) (probe (position 106 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors"))) (kind memberAccessOperand) (ordinal 1) (authored-target "earlierOccurrence::successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 104 30) (end 104 40)) (probe (position 104 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 104 52) (end 104 56)) (probe (position 104 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::earlierOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 105 28) (end 105 38)) (probe (position 105 28))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 105 50) (end 105 54)) (probe (position 105 50))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::successors::laterOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 77 27) (end 77 37)) (probe (position 77 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 77 52) (end 77 63)) (probe (position 77 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::superoccurrence"))) (kind subsetting) (ordinal 0) (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 597 35) (end 597 45)) (probe (position 597 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 597 61) (end 597 81)) (probe (position 597 61))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "outsideOfOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 603 28) (end 603 38)) (probe (position 603 28))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 603 51) (end 603 55)) (probe (position 603 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 604 29) (end 604 39)) (probe (position 604 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 604 52) (end 604 56)) (probe (position 604 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences::surroundingSpace"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 41 17) (end 41 27)) (probe (position 41 17))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 41 39) (end 41 43)) (probe (position 41 39))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (kind expressionOperand) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 150 41) (end 150 51)) (probe (position 150 41))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 150 66) (end 150 89)) (probe (position 150 66))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "timeEnclosedOccurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 166 3) (end 166 9)) (probe (position 166 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 167 4) (end 167 11)) (probe (position 167 4))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 166 10) (end 166 50)) (probe (position 166 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 0) (authored-target "thatOccurrence::timeCoincidentOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 167 12) (end 167 52)) (probe (position 167 12))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences"))) (kind memberAccessOperand) (ordinal 1) (authored-target "thisOccurrence::timeCoincidentOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 157 27) (end 157 37)) (probe (position 157 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 157 49) (end 157 65)) (probe (position 157 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "longerOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 158 27) (end 158 37)) (probe (position 158 27))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 158 49) (end 158 66)) (probe (position 158 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "shorterOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 127 35) (end 127 45)) (probe (position 127 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 127 60) (end 127 71)) (probe (position 127 60))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind subsetting) (ordinal 0) (authored-target "occurrences")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 143 3) (end 143 9)) (probe (position 143 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 143 40) (end 143 47)) (probe (position 143 40))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 144 3) (end 144 9)) (probe (position 144 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 2) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 144 38) (end 144 45)) (probe (position 144 38))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 3) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 147 3) (end 147 9)) (probe (position 147 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 4) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 147 52) (end 147 59)) (probe (position 147 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind expressionOperand) (ordinal 5) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 143 10) (end 143 39)) (probe (position 143 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 0) (authored-target "longerOccurrence::predecessors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 143 48) (end 143 78)) (probe (position 143 48))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 1) (authored-target "shorterOccurrence::predecessors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 144 10) (end 144 37)) (probe (position 144 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 2) (authored-target "longerOccurrence::successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 144 46) (end 144 74)) (probe (position 144 46))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 3) (authored-target "shorterOccurrence::successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 147 10) (end 147 51)) (probe (position 147 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 4) (authored-target "shorterOccurrence::timeEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 147 60) (end 147 100)) (probe (position 147 60))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))) (kind memberAccessOperand) (ordinal 5) (authored-target "longerOccurrence::timeEnclosedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 141 29) (end 141 39)) (probe (position 141 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 141 51) (end 141 55)) (probe (position 141 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 142 30) (end 142 40)) (probe (position 142 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 142 52) (end 142 56)) (probe (position 142 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 316 24) (end 316 34)) (probe (position 316 24))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 316 49) (end 316 58)) (probe (position 316 49))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind subsetting) (ordinal 0) (authored-target "portionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 327 3) (end 327 9)) (probe (position 327 3))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind expressionOperand) (ordinal 0) (authored-target "subset")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 327 43) (end 327 50)) (probe (position 327 43))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind expressionOperand) (ordinal 1) (authored-target "subsets")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 327 10) (end 327 42)) (probe (position 327 10))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind memberAccessOperand) (ordinal 0) (authored-target "timeSlicedOccurrence::timeSliceOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 327 51) (end 327 82)) (probe (position 327 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf"))) (kind memberAccessOperand) (ordinal 1) (authored-target "timeSliceOccurrence::timeSliceOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 323 32) (end 323 42)) (probe (position 323 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 323 54) (end 323 58)) (probe (position 323 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSliceOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 324 33) (end 324 43)) (probe (position 324 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 324 55) (end 324 59)) (probe (position 324 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSliceOf::timeSlicedOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "self")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::self")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 308 30) (end 308 40)) (probe (position 308 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 308 55) (end 308 63)) (probe (position 308 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))) (kind subsetting) (ordinal 0) (authored-target "portions")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 438 20) (end 438 23)) (probe (position 438 20))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf"))) (kind featureTyping) (ordinal 0) (authored-target "Set")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 447 31) (end 447 41)) (probe (position 447 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 447 21) (end 447 29)) (probe (position 447 21))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 448 18) (end 448 28)) (probe (position 448 18))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::unionsOf::union"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 79 30) (end 79 40)) (probe (position 79 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence::withoutOccurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 931 33) (end 931 42)) (probe (position 931 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind specialization) (ordinal 0) (authored-target "SpaceLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 931 44) (end 931 51)) (probe (position 931 44))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf"))) (kind specialization) (ordinal 1) (authored-target "Without")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 941 29) (end 941 39)) (probe (position 941 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 941 50) (end 941 66)) (probe (position 941 50))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind redefinition) (ordinal 0) (authored-target "targetOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 941 68) (end 941 86)) (probe (position 941 68))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace"))) (kind redefinition) (ordinal 1) (authored-target "separateOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 939 32) (end 939 42)) (probe (position 939 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 939 53) (end 939 69)) (probe (position 939 53))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind redefinition) (ordinal 0) (authored-target "sourceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 939 71) (end 939 92)) (probe (position 939 71))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo"))) (kind redefinition) (ordinal 1) (authored-target "separateOccurrenceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 823 33) (end 823 39)) (probe (position 823 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf"))) (kind specialization) (ordinal 0) (authored-target "Within")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 829 33) (end 829 43)) (probe (position 829 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 829 54) (end 829 71)) (probe (position 829 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "smallerOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 728 36) (end 728 52)) (probe (position 728 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfLink"))) (kind specialization) (ordinal 0) (authored-target "SelfSameLifeLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 708 40) (end 708 50)) (probe (position 708 40))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink"))) (kind specialization) (ordinal 0) (authored-target "BinaryLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 723 32) (end 723 41)) (probe (position 723 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceDataValue"))) (kind featureTyping) (ordinal 0) (authored-target "DataValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 723 57) (end 723 71)) (probe (position 723 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceDataValue"))) (kind subsetting) (ordinal 0) (authored-target "myselfSameLife")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 719 33) (end 719 43)) (probe (position 719 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 719 59) (end 719 73)) (probe (position 719 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::sourceOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "myselfSameLife")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 724 32) (end 724 41)) (probe (position 724 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetDataValue"))) (kind featureTyping) (ordinal 0) (authored-target "DataValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 724 57) (end 724 69)) (probe (position 724 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetDataValue"))) (kind subsetting) (ordinal 0) (authored-target "selfSameLife")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 720 33) (end 720 43)) (probe (position 720 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 720 59) (end 720 71)) (probe (position 720 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind subsetting) (ordinal 0) (authored-target "selfSameLife")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 720 73) (end 720 109)) (probe (position 720 73))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SelfSameLifeLink::targetOccurrence"))) (kind subsetting) (ordinal 1) (authored-target "sourceOccurrence::sameLifeOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 843 34) (end 843 45)) (probe (position 843 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf"))) (kind specialization) (ordinal 0) (authored-target "TimeSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 849 34) (end 849 44)) (probe (position 849 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 849 55) (end 849 74)) (probe (position 849 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SnapshotOf::snapshotOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "timeSliceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 769 29) (end 769 39)) (probe (position 769 29))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink"))) (kind specialization) (ordinal 0) (authored-target "BinaryLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 779 38) (end 779 48)) (probe (position 779 38))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 779 59) (end 779 77)) (probe (position 779 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::sourceOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 780 38) (end 780 48)) (probe (position 780 38))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 780 59) (end 780 77)) (probe (position 780 59))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceLink::targetOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 866 35) (end 866 47)) (probe (position 866 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf"))) (kind specialization) (ordinal 0) (authored-target "SpaceSliceOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 873 35) (end 873 45)) (probe (position 873 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 873 56) (end 873 76)) (probe (position 873 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceShotOf::spaceShotOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "spaceSliceOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 853 36) (end 853 45)) (probe (position 853 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf"))) (kind specialization) (ordinal 0) (authored-target "PortionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 862 36) (end 862 46)) (probe (position 862 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 862 57) (end 862 74)) (probe (position 862 57))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SpaceSliceOf::spaceSliceOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "portionOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 981 36) (end 981 45)) (probe (position 981 36))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy"))) (kind specialization) (ordinal 0) (authored-target "OutsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 988 31) (end 988 41)) (probe (position 988 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 988 52) (end 988 68)) (probe (position 988 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundedSpace"))) (kind redefinition) (ordinal 0) (authored-target "separateSpaceToo")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpaceToo")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 989 32) (end 989 42)) (probe (position 989 32))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 989 53) (end 989 66)) (probe (position 989 53))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::SurroundedBy::surroundingSpace"))) (kind redefinition) (ordinal 0) (authored-target "separateSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::OutsideOf::separateSpace")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 833 35) (end 833 44)) (probe (position 833 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf"))) (kind specialization) (ordinal 0) (authored-target "PortionOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 839 35) (end 839 45)) (probe (position 839 35))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 839 56) (end 839 73)) (probe (position 839 56))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::TimeSliceOf::timeSliceOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "portionOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::PortionOf::portionOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 796 30) (end 796 43)) (probe (position 796 30))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind specialization) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 796 45) (end 796 53)) (probe (position 796 45))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within"))) (kind specialization) (ordinal 1) (authored-target "InsideOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 805 33) (end 805 43)) (probe (position 805 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 805 54) (end 805 71)) (probe (position 805 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "shorterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensDuring::shorterOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 805 73) (end 805 85)) (probe (position 805 73))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence"))) (kind redefinition) (ordinal 1) (authored-target "smallerSpace")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::InsideOf::smallerSpace")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 810 34) (end 810 40)) (probe (position 810 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind specialization) (ordinal 0) (authored-target "Within")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 810 42) (end 810 54)) (probe (position 810 42))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth"))) (kind specialization) (ordinal 1) (authored-target "HappensWhile")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 818 39) (end 818 56)) (probe (position 818 39))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "smallerOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Within::smallerOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 818 58) (end 818 86)) (probe (position 818 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::WithinBoth::thisOccurrence"))) (kind redefinition) (ordinal 1) (authored-target "HappensWhile::thisOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensWhile::thisOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 877 31) (end 877 41)) (probe (position 877 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without"))) (kind specialization) (ordinal 0) (authored-target "BinaryLink")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 887 34) (end 887 44)) (probe (position 887 34))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 887 55) (end 887 73)) (probe (position 887 55))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 885 37) (end 885 47)) (probe (position 885 37))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 885 58) (end 885 76)) (probe (position 885 58))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Without::separateOccurrenceToo"))) (kind redefinition) (ordinal 0) (authored-target "BinaryLink::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 704 41) (end 704 61)) (probe (position 704 41))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort"))) (kind featureTyping) (ordinal 0) (authored-target "IncomingTransferSort")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 705 28) (end 705 49)) (probe (position 705 28))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind memberAccessOperand) (ordinal 0) (authored-target "t1::endShot::successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 705 51) (end 705 61)) (probe (position 705 51))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind memberAccessOperand) (ordinal 1) (authored-target "t2::endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 705 19) (end 705 27)) (probe (position 705 19))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort::t1First"))) (kind invocationCallee) (ordinal 0) (authored-target "includes")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 920 33) (end 920 46)) (probe (position 920 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (kind featureTyping) (ordinal 0) (authored-target "HappensBefore")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 920 71) (end 920 82)) (probe (position 920 71))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (kind subsetting) (ordinal 0) (authored-target "binaryLinks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 927 33) (end 927 43)) (probe (position 927 33))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 927 54) (end 927 86)) (probe (position 927 54))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "HappensBefore::earlierOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::earlierOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 927 88) (end 927 107)) (probe (position 927 88))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::earlierOccurrence"))) (kind redefinition) (ordinal 1) (authored-target "binaryLinks::source")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 928 31) (end 928 41)) (probe (position 928 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 928 52) (end 928 82)) (probe (position 928 52))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind redefinition) (ordinal 0) (authored-target "HappensBefore::laterOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::HappensBefore::laterOccurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 928 84) (end 928 103)) (probe (position 928 84))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks::laterOccurrence"))) (kind redefinition) (ordinal 1) (authored-target "binaryLinks::target")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 696 31) (end 696 41)) (probe (position 696 31))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::Occurrence")))))
  )
  (query (document "memory://snapshot/occurrences.md") (range (start 696 66) (end 696 72)) (probe (position 696 66))
    (reference (id (source (node (document "memory://snapshot/occurrences.md") (qualified-name "Occurrences::occurrences"))) (kind subsetting) (ordinal 0) (authored-target "things")
      (outcome (status unresolved)))
  )
)
~~~
