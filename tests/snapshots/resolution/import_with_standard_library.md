# META
~~~ini
description=Import plans preserve the complete settled standard-library semantics
type=file
libraries=standard
~~~
# SOURCE
## model.sysml
~~~sysml
package Catalog { part def Motor; }
package Machine { part drive : Mot; part other : Motor; }
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 1 32 imports=Mot fix-imports
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 31) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 1 49) (end 1 54))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d6b61df0470a002863049c2b3dea8f0cb57c807b53a7d54340df946a41c1a3b3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::drive"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mot")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::drive"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::drive"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Catalog::Motor")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::drive")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 31) (end 1 34)) (probe (position 1 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::drive"))) (kind featureTyping) (ordinal 0) (authored-target "Mot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 1 49) (end 1 54)) (probe (position 1 49))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::other"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 1 32)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Actions") (qualified-name "Actions") (kind "LibraryPackage")) (member (name "Allocations") (qualified-name "Allocations") (kind "LibraryPackage")) (member (name "AnalysisCases") (qualified-name "AnalysisCases") (kind "LibraryPackage")) (member (name "AnalysisTooling") (qualified-name "AnalysisTooling") (kind "LibraryPackage")) (member (name "Attributes") (qualified-name "Attributes") (kind "LibraryPackage")) (member (name "Base") (qualified-name "Base") (kind "LibraryPackage")) (member (name "BaseFunctions") (qualified-name "BaseFunctions") (kind "LibraryPackage")) (member (name "BooleanFunctions") (qualified-name "BooleanFunctions") (kind "LibraryPackage")) (member (name "Calculations") (qualified-name "Calculations") (kind "LibraryPackage")) (member (name "Cases") (qualified-name "Cases") (kind "LibraryPackage")) (member (name "Catalog") (qualified-name "Catalog") (kind "Package")) (member (name "CausationConnections") (qualified-name "CausationConnections") (kind "LibraryPackage")) (member (name "CauseAndEffect") (qualified-name "CauseAndEffect") (kind "LibraryPackage")) (member (name "Clocks") (qualified-name "Clocks") (kind "LibraryPackage")) (member (name "CollectionFunctions") (qualified-name "CollectionFunctions") (kind "LibraryPackage")) (member (name "Collections") (qualified-name "Collections") (kind "LibraryPackage")) (member (name "ComplexFunctions") (qualified-name "ComplexFunctions") (kind "LibraryPackage")) (member (name "Connections") (qualified-name "Connections") (kind "LibraryPackage")) (member (name "Constraints") (qualified-name "Constraints") (kind "LibraryPackage")) (member (name "ControlFunctions") (qualified-name "ControlFunctions") (kind "LibraryPackage")) (member (name "ControlPerformances") (qualified-name "ControlPerformances") (kind "LibraryPackage")) (member (name "DataFunctions") (qualified-name "DataFunctions") (kind "LibraryPackage")) (member (name "DerivationConnections") (qualified-name "DerivationConnections") (kind "LibraryPackage")) (member (name "FeatureReferencingPerformances") (qualified-name "FeatureReferencingPerformances") (kind "LibraryPackage")) (member (name "Flows") (qualified-name "Flows") (kind "LibraryPackage")) (member (name "ISQ") (qualified-name "ISQ") (kind "LibraryPackage")) (member (name "ISQAcoustics") (qualified-name "ISQAcoustics") (kind "LibraryPackage")) (member (name "ISQAtomicNuclear") (qualified-name "ISQAtomicNuclear") (kind "LibraryPackage")) (member (name "ISQBase") (qualified-name "ISQBase") (kind "LibraryPackage")) (member (name "ISQCharacteristicNumbers") (qualified-name "ISQCharacteristicNumbers") (kind "LibraryPackage")) (member (name "ISQChemistryMolecular") (qualified-name "ISQChemistryMolecular") (kind "LibraryPackage")) (member (name "ISQCondensedMatter") (qualified-name "ISQCondensedMatter") (kind "LibraryPackage")) (member (name "ISQElectromagnetism") (qualified-name "ISQElectromagnetism") (kind "LibraryPackage")) (member (name "ISQInformation") (qualified-name "ISQInformation") (kind "LibraryPackage")) (member (name "ISQLight") (qualified-name "ISQLight") (kind "LibraryPackage")) (member (name "ISQMechanics") (qualified-name "ISQMechanics") (kind "LibraryPackage")) (member (name "ISQSpaceTime") (qualified-name "ISQSpaceTime") (kind "LibraryPackage")) (member (name "ISQThermodynamics") (qualified-name "ISQThermodynamics") (kind "LibraryPackage")) (member (name "ImageMetadata") (qualified-name "ImageMetadata") (kind "LibraryPackage")) (member (name "IntegerFunctions") (qualified-name "IntegerFunctions") (kind "LibraryPackage")) (member (name "Interfaces") (qualified-name "Interfaces") (kind "LibraryPackage")) (member (name "Items") (qualified-name "Items") (kind "LibraryPackage")) (member (name "KerML") (qualified-name "KerML") (kind "LibraryPackage")) (member (name "Links") (qualified-name "Links") (kind "LibraryPackage")) (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "MeasurementRefCalculations") (qualified-name "MeasurementRefCalculations") (kind "LibraryPackage")) (member (name "MeasurementReferences") (qualified-name "MeasurementReferences") (kind "LibraryPackage")) (member (name "Metadata") (qualified-name "Metadata") (kind "LibraryPackage")) (member (name "Metaobjects") (qualified-name "Metaobjects") (kind "LibraryPackage")) (member (name "ModelingMetadata") (qualified-name "ModelingMetadata") (kind "LibraryPackage")) (member (name "NaturalFunctions") (qualified-name "NaturalFunctions") (kind "LibraryPackage")) (member (name "NumericalFunctions") (qualified-name "NumericalFunctions") (kind "LibraryPackage")) (member (name "Objects") (qualified-name "Objects") (kind "LibraryPackage")) (member (name "Observation") (qualified-name "Observation") (kind "LibraryPackage")) (member (name "OccurrenceFunctions") (qualified-name "OccurrenceFunctions") (kind "LibraryPackage")) (member (name "Occurrences") (qualified-name "Occurrences") (kind "LibraryPackage")) (member (name "ParametersOfInterestMetadata") (qualified-name "ParametersOfInterestMetadata") (kind "LibraryPackage")) (member (name "Parts") (qualified-name "Parts") (kind "LibraryPackage")) (member (name "Performances") (qualified-name "Performances") (kind "LibraryPackage")) (member (name "Ports") (qualified-name "Ports") (kind "LibraryPackage")) (member (name "Quantities") (qualified-name "Quantities") (kind "LibraryPackage")) (member (name "QuantityCalculations") (qualified-name "QuantityCalculations") (kind "LibraryPackage")) (member (name "RationalFunctions") (qualified-name "RationalFunctions") (kind "LibraryPackage")) (member (name "RealFunctions") (qualified-name "RealFunctions") (kind "LibraryPackage")) (member (name "RequirementDerivation") (qualified-name "RequirementDerivation") (kind "LibraryPackage")) (member (name "Requirements") (qualified-name "Requirements") (kind "LibraryPackage")) (member (name "RiskMetadata") (qualified-name "RiskMetadata") (kind "LibraryPackage")) (member (name "SI") (qualified-name "SI") (kind "LibraryPackage")) (member (name "SIPrefixes") (qualified-name "SIPrefixes") (kind "LibraryPackage")) (member (name "SampledFunctions") (qualified-name "SampledFunctions") (kind "LibraryPackage")) (member (name "ScalarFunctions") (qualified-name "ScalarFunctions") (kind "LibraryPackage")) (member (name "ScalarValues") (qualified-name "ScalarValues") (kind "LibraryPackage")) (member (name "SequenceFunctions") (qualified-name "SequenceFunctions") (kind "LibraryPackage")) (member (name "ShapeItems") (qualified-name "ShapeItems") (kind "LibraryPackage")) (member (name "SpatialFrames") (qualified-name "SpatialFrames") (kind "LibraryPackage")) (member (name "SpatialItems") (qualified-name "SpatialItems") (kind "LibraryPackage")) (member (name "StandardViewDefinitions") (qualified-name "StandardViewDefinitions") (kind "LibraryPackage")) (member (name "StatePerformances") (qualified-name "StatePerformances") (kind "LibraryPackage")) (member (name "StateSpaceRepresentation") (qualified-name "StateSpaceRepresentation") (kind "LibraryPackage")) (member (name "States") (qualified-name "States") (kind "LibraryPackage")) (member (name "StringFunctions") (qualified-name "StringFunctions") (kind "LibraryPackage")) (member (name "SysML") (qualified-name "SysML") (kind "LibraryPackage")) (member (name "TensorCalculations") (qualified-name "TensorCalculations") (kind "LibraryPackage")) (member (name "Time") (qualified-name "Time") (kind "LibraryPackage")) (member (name "TradeStudies") (qualified-name "TradeStudies") (kind "LibraryPackage")) (member (name "Transfers") (qualified-name "Transfers") (kind "LibraryPackage")) (member (name "TransitionPerformances") (qualified-name "TransitionPerformances") (kind "LibraryPackage")) (member (name "TrigFunctions") (qualified-name "TrigFunctions") (kind "LibraryPackage")) (member (name "Triggers") (qualified-name "Triggers") (kind "LibraryPackage")) (member (name "USCustomaryUnits") (qualified-name "USCustomaryUnits") (kind "LibraryPackage")) (member (name "UseCases") (qualified-name "UseCases") (kind "LibraryPackage")) (member (name "VectorCalculations") (qualified-name "VectorCalculations") (kind "LibraryPackage")) (member (name "VectorFunctions") (qualified-name "VectorFunctions") (kind "LibraryPackage")) (member (name "VectorValues") (qualified-name "VectorValues") (kind "LibraryPackage")) (member (name "VerificationCases") (qualified-name "VerificationCases") (kind "LibraryPackage")) (member (name "Views") (qualified-name "Views") (kind "LibraryPackage")) (member (name "boundingShapes") (qualified-name "Items::Item::boundingShapes") (kind "ItemUsage")) (member (name "checkedConstraints") (qualified-name "Items::Item::checkedConstraints") (kind "ConstraintUsage")) (member (name "differencesOf") (qualified-name "Occurrences::Occurrence::differencesOf") (kind "Feature")) (member (name "dispatchScope") (qualified-name "Occurrences::Occurrence::dispatchScope") (kind "Feature")) (member (name "done") (qualified-name "Parts::Part::done") (kind "PartUsage")) (member (name "drive") (qualified-name "Machine::drive") (kind "PartUsage")) (member (name "enactedPerformances") (qualified-name "Objects::Object::enactedPerformances") (kind "Step")) (member (name "endShot") (qualified-name "Occurrences::Occurrence::endShot") (kind "Feature")) (member (name "envelopingShapes") (qualified-name "Items::Item::envelopingShapes") (kind "ItemUsage")) (member (name "exhibitedStates") (qualified-name "Parts::Part::exhibitedStates") (kind "StateUsage")) (member (name "immediatePredecessors") (qualified-name "Occurrences::Occurrence::immediatePredecessors") (kind "Feature")) (member (name "immediateSuccessors") (qualified-name "Occurrences::Occurrence::immediateSuccessors") (kind "Feature")) (member (name "incomingTransferSort") (qualified-name "Occurrences::Occurrence::incomingTransferSort") (kind "Feature")) (member (name "incomingTransfers") (qualified-name "Occurrences::Occurrence::incomingTransfers") (kind "Feature")) (member (name "incomingTransfersToSelf") (qualified-name "Occurrences::Occurrence::incomingTransfersToSelf") (kind "Feature")) (member (name "innerSpaceDimension") (qualified-name "Occurrences::Occurrence::innerSpaceDimension") (kind "Feature")) (member (name "innerSpaceOccurrences") (qualified-name "Occurrences::Occurrence::innerSpaceOccurrences") (kind "Feature")) (member (name "intersectionsOf") (qualified-name "Occurrences::Occurrence::intersectionsOf") (kind "Feature")) (member (name "involvingPerformances") (qualified-name "Objects::Object::involvingPerformances") (kind "Feature")) (member (name "isClosed") (qualified-name "Occurrences::Occurrence::isClosed") (kind "Feature")) (member (name "isDispatch") (qualified-name "Occurrences::Occurrence::isDispatch") (kind "Feature")) (member (name "isRunToCompletion") (qualified-name "Occurrences::Occurrence::isRunToCompletion") (kind "Feature")) (member (name "isSolid") (qualified-name "Items::Item::isSolid") (kind "AttributeUsage")) (member (name "justOutsideOfOccurrences") (qualified-name "Occurrences::Occurrence::justOutsideOfOccurrences") (kind "Feature")) (member (name "localClock") (qualified-name "Occurrences::Occurrence::localClock") (kind "Feature")) (member (name "matingOccurrences") (qualified-name "Occurrences::Occurrence::matingOccurrences") (kind "Feature")) (member (name "middleTimeSlice") (qualified-name "Occurrences::Occurrence::middleTimeSlice") (kind "Feature")) (member (name "other") (qualified-name "Machine::other") (kind "PartUsage")) (member (name "outerSpaceDimension") (qualified-name "Occurrences::Occurrence::outerSpaceDimension") (kind "Feature")) (member (name "outgoingTransfers") (qualified-name "Occurrences::Occurrence::outgoingTransfers") (kind "Feature")) (member (name "outgoingTransfersFromSelf") (qualified-name "Occurrences::Occurrence::outgoingTransfersFromSelf") (kind "Feature")) (member (name "outsideOfOccurrences") (qualified-name "Occurrences::Occurrence::outsideOfOccurrences") (kind "Feature")) (member (name "ownedActions") (qualified-name "Parts::Part::ownedActions") (kind "ActionUsage")) (member (name "ownedPerformances") (qualified-name "Objects::Object::ownedPerformances") (kind "Step")) (member (name "ownedPorts") (qualified-name "Parts::Part::ownedPorts") (kind "PortUsage")) (member (name "ownedStates") (qualified-name "Parts::Part::ownedStates") (kind "StateUsage")) (member (name "performedActions") (qualified-name "Parts::Part::performedActions") (kind "ActionUsage")) (member (name "portionOf") (qualified-name "Occurrences::Occurrence::portionOf") (kind "Feature")) (member (name "portionOfLife") (qualified-name "Occurrences::Occurrence::portionOfLife") (kind "Feature")) (member (name "portions") (qualified-name "Occurrences::Occurrence::portions") (kind "Feature")) (member (name "predecessors") (qualified-name "Occurrences::Occurrence::predecessors") (kind "Feature")) (member (name "runToCompletionScope") (qualified-name "Occurrences::Occurrence::runToCompletionScope") (kind "Feature")) (member (name "sameLifeOccurrences") (qualified-name "Occurrences::Occurrence::sameLifeOccurrences") (kind "Feature")) (member (name "self") (qualified-name "Parts::Part::self") (kind "ReferenceUsage")) (member (name "shape") (qualified-name "Items::Item::shape") (kind "ItemUsage")) (member (name "snapshotOf") (qualified-name "Occurrences::Occurrence::snapshotOf") (kind "Feature")) (member (name "snapshots") (qualified-name "Occurrences::Occurrence::snapshots") (kind "Feature")) (member (name "spaceBoundary") (qualified-name "Occurrences::Occurrence::spaceBoundary") (kind "Feature")) (member (name "spaceBoundaryOf") (qualified-name "Occurrences::Occurrence::spaceBoundaryOf") (kind "Feature")) (member (name "spaceEnclosedOccurrences") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences") (kind "Feature")) (member (name "spaceInterior") (qualified-name "Occurrences::Occurrence::spaceInterior") (kind "Feature")) (member (name "spaceInteriorOf") (qualified-name "Occurrences::Occurrence::spaceInteriorOf") (kind "Feature")) (member (name "spaceShotOf") (qualified-name "Occurrences::Occurrence::spaceShotOf") (kind "Feature")) (member (name "spaceShots") (qualified-name "Occurrences::Occurrence::spaceShots") (kind "Feature")) (member (name "spaceSliceOf") (qualified-name "Occurrences::Occurrence::spaceSliceOf") (kind "Feature")) (member (name "spaceSlices") (qualified-name "Occurrences::Occurrence::spaceSlices") (kind "Feature")) (member (name "spaceTimeCoincidentOccurrences") (qualified-name "Occurrences::Occurrence::spaceTimeCoincidentOccurrences") (kind "Feature")) (member (name "spaceTimeEnclosedOccurrences") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences") (kind "Feature")) (member (name "spaceTimeEnclosedPoints") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedPoints") (kind "Feature")) (member (name "start") (qualified-name "Parts::Part::start") (kind "PartUsage")) (member (name "startShot") (qualified-name "Occurrences::Occurrence::startShot") (kind "Feature")) (member (name "structuredSpaceBoundary") (qualified-name "Objects::Object::structuredSpaceBoundary") (kind "Feature")) (member (name "subitems") (qualified-name "Items::Item::subitems") (kind "ItemUsage")) (member (name "subobjects") (qualified-name "Objects::Object::subobjects") (kind "Feature")) (member (name "suboccurrences") (qualified-name "Occurrences::Occurrence::suboccurrences") (kind "Feature")) (member (name "subparts") (qualified-name "Items::Item::subparts") (kind "PartUsage")) (member (name "successors") (qualified-name "Occurrences::Occurrence::successors") (kind "Feature")) (member (name "superoccurrence") (qualified-name "Occurrences::Occurrence::superoccurrence") (kind "Feature")) (member (name "surroundedByOccurrences") (qualified-name "Occurrences::Occurrence::surroundedByOccurrences") (kind "Feature")) (member (name "that") (qualified-name "Base::things::that") (kind "Feature")) (member (name "this") (qualified-name "Occurrences::Occurrence::this") (kind "Feature")) (member (name "timeCoincidentOccurrences") (qualified-name "Occurrences::Occurrence::timeCoincidentOccurrences") (kind "Feature")) (member (name "timeEnclosedOccurrences") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences") (kind "Feature")) (member (name "timeSliceOf") (qualified-name "Occurrences::Occurrence::timeSliceOf") (kind "Feature")) (member (name "timeSlices") (qualified-name "Occurrences::Occurrence::timeSlices") (kind "Feature")) (member (name "unionsOf") (qualified-name "Occurrences::Occurrence::unionsOf") (kind "Feature")) (member (name "voids") (qualified-name "Items::Item::voids") (kind "ItemUsage")) (member (name "withoutOccurrences") (qualified-name "Occurrences::Occurrence::withoutOccurrences") (kind "Feature"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "drive")
          (qualified-name "Machine::drive")
          (location (document "memory://snapshot/model.sysml") (range (start 1 23) (end 1 28)) (role Declaration))
          (declaration (range (start 1 18) (end 1 35)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Mot") (target unresolved))
          (relationship (kind "subsetting") (provenance implied) (target resolved))
          (typing (outcome unresolved))
          (effective-typing (outcome partial) (type (qualified-name "Base::Anything") (origin inherited) (provenance implied)) (type (qualified-name "Items::Item") (origin inherited) (provenance implied)) (type (qualified-name "Parts::Part") (origin inherited) (provenance implied)) (type (qualified-name "Objects::Object") (origin inherited) (provenance implied)) (type (qualified-name "Occurrences::Occurrence") (origin inherited) (provenance implied)))
          (outgoing (kind "subsetting") (peer "Parts::parts") (provenance implied))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
    (completion-imports (status resolved) (import (target "Catalog::Motor") (referent "Catalog::Motor") (spelling "Motor") (namespace "Machine") (range (start 1 31) (end 1 34))) (safe true))
    (missing-imports (status resolved) (import (target "Catalog::Motor") (referent "Catalog::Motor") (spelling "Motor") (namespace "Machine") (range (start 1 49) (end 1 54))) (repaired (range (start 1 49) (end 1 54))) (remaining (Unavailable (range (start 1 31) (end 1 34)))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "Catalog") (qualified-name "Catalog") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 15)) (role Declaration)) (declaration (range (start 0 0) (end 0 35))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "Catalog::Motor") (location (document "memory://snapshot/model.sysml") (range (start 0 27) (end 0 32)) (role Declaration)) (declaration (range (start 0 18) (end 0 33))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 15)) (role Declaration)) (declaration (range (start 1 0) (end 1 57))))
    (symbol (kind "PartUsage") (name "drive") (qualified-name "Machine::drive") (location (document "memory://snapshot/model.sysml") (range (start 1 23) (end 1 28)) (role Declaration)) (declaration (range (start 1 18) (end 1 35))))
    (symbol (kind "PartUsage") (name "other") (qualified-name "Machine::other") (location (document "memory://snapshot/model.sysml") (range (start 1 41) (end 1 46)) (role Declaration)) (declaration (range (start 1 36) (end 1 55))))
  )
)
~~~
