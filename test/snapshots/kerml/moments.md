# META
~~~ini
description=KerML Enhancements: Moments
type=file
~~~
# SOURCE
~~~kerml
package Moments {
    private import Occurrences::Life;
    private import Occurrences::Occurrence;

    class Eternity specializes Life {
        // Nothing before/after or outside.
        // Could be many of these, see universal below.
        redefines predecessors [0];
        redefines successors [0];
        redefines outsideOfOccurrences [0];
    }

    class UniversalEternity [1] specializes Eternity {
      redefines timeSlices: Period;  //Includes life.
      redefines snapshots : Moment;
    }

    feature universalEternity : UniversalEternity [1];

    class Period {  //Includes life and snapshots.
      //↓↓ With UE redef, exactly UE timeslices.
      redefines timeSliceOf : UniversalEternity [1];
    }

    class all InstantOccurrence specializes Occurrence {
        // Probly useful elsewhere, eg, to type snapshots.
        redefines snapshots [1]; // Or startShot subsets endShot;
    }                            // Or middleTimeslice [0];

    class Moment specializes Period, InstantOccurrence {
      //↓↓ With UE redef, exactly UE snapshots.
      redefines snapshotOf : UniversalEternity [1]; }
      
    private import Occurrence::spaceTimeCoincidentOccurrences;
    //UE portion "corresponding" to an occurrence.
    feature coincidentUEPortion : Occurrence [1] subsets spaceTimeCoincidentOccurrences,
                                                         universalEternity.portions
                                                 featured by Occurrence;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "moments.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 19) (end 33 61))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package Moments {
    private import Occurrences::Life;
    private import Occurrences::Occurrence;

    class Eternity specializes Life {
        // Nothing before/after or outside.
        // Could be many of these, see universal below.
        redefines predecessors [0];
        redefines successors [0];
        redefines outsideOfOccurrences [0];
    }

    class UniversalEternity [1] specializes Eternity {
      redefines timeSlices: Period;  //Includes life.
      redefines snapshots : Moment;
    }

    feature universalEternity : UniversalEternity [1];

    class Period {  //Includes life and snapshots.
      //↓↓ With UE redef, exactly UE timeslices.
      redefines timeSliceOf : UniversalEternity [1];
    }

    class all InstantOccurrence specializes Occurrence {
        // Probly useful elsewhere, eg, to type snapshots.
        redefines snapshots [1]; // Or startShot subsets endShot;
    }                            // Or middleTimeslice [0];

    class Moment specializes Period, InstantOccurrence {
      //↓↓ With UE redef, exactly UE snapshots.
      redefines snapshotOf : UniversalEternity [1]; }
      
    private import Occurrence::spaceTimeCoincidentOccurrences;
    //UE portion "corresponding" to an occurrence.
    feature coincidentUEPortion : Occurrence [1] subsets spaceTimeCoincidentOccurrences,
                                                         universalEternity.portions
                                                 featured by Occurrence;
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4073a10b459d571fdeecdfd3082c66de4cffa55a2f213007e2b7422792e3ae48") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Moments"))) (kind "package") (name "Moments") (declared-name "Moments"))
    (element (id (node (document "d0") (qualified-name "Moments::Eternity"))) (kind "classifier decl") (name "Eternity") (declared-name "Eternity") (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::Life"))) (kind "import") (name "Life") (declared-name "Life") (parent (node (document "d0") (qualified-name "Moments"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Life") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Moments::Moment"))) (kind "classifier decl") (name "Moment") (declared-name "Moment") (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Moments"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Moments::Period"))) (kind "classifier decl") (name "Period") (declared-name "Period") (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::UniversalEternity"))) (kind "classifier decl") (name "UniversalEternity") (declared-name "UniversalEternity") (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::all"))) (kind "classifier decl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::coincidentUEPortion"))) (kind "feature decl") (name "coincidentUEPortion") (declared-name "coincidentUEPortion") (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::spaceTimeCoincidentOccurrences"))) (kind "import") (name "spaceTimeCoincidentOccurrences") (declared-name "spaceTimeCoincidentOccurrences") (parent (node (document "d0") (qualified-name "Moments"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrence::spaceTimeCoincidentOccurrences") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Moments::universalEternity"))) (kind "feature decl") (name "universalEternity") (declared-name "universalEternity") (parent (node (document "d0") (qualified-name "Moments"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Moments::Life"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Life") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Moments::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Moments::spaceTimeCoincidentOccurrences"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrence::spaceTimeCoincidentOccurrences") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 1 19) (end 1 36)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "Moments::Life"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
        (range (start 1 19) (end 1 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 19) (end 2 42)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "Moments::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 2 19) (end 2 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 19) (end 33 61)) (probe (position 33 19))
      (reference
        (source (document "d0") (qualified-name "Moments::spaceTimeCoincidentOccurrences"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrence::spaceTimeCoincidentOccurrences")
        (range (start 33 19) (end 33 61))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
