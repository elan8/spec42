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
    (element (id (node (document "d0") (qualified-name "Moments"))) (kind "package") (name "Moments") (declared-name "Moments") (range (start (line 0) (character 0)) (end (line 0) (character 1506))))
    (element (id (node (document "d0") (qualified-name "Moments::Eternity"))) (kind "classifier decl") (name "Eternity") (declared-name "Eternity") (range (start (line 4) (character 4)) (end (line 4) (character 257))) (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::Life"))) (kind "import") (name "Life") (declared-name "Life") (range (start (line 1) (character 4)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "Moments"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Life") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Moments::Moment"))) (kind "classifier decl") (name "Moment") (declared-name "Moment") (range (start (line 29) (character 4)) (end (line 29) (character 162))) (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 2) (character 4)) (end (line 2) (character 43))) (parent (node (document "d0") (qualified-name "Moments"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 42))))))
    (element (id (node (document "d0") (qualified-name "Moments::Period"))) (kind "classifier decl") (name "Period") (declared-name "Period") (range (start (line 19) (character 4)) (end (line 19) (character 162))) (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::UniversalEternity"))) (kind "classifier decl") (name "UniversalEternity") (declared-name "UniversalEternity") (range (start (line 12) (character 4)) (end (line 12) (character 150))) (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::all"))) (kind "classifier decl") (name "all") (declared-name "all") (range (start (line 24) (character 4)) (end (line 24) (character 187))) (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::coincidentUEPortion"))) (kind "feature decl") (name "coincidentUEPortion") (declared-name "coincidentUEPortion") (range (start (line 35) (character 4)) (end (line 35) (character 245))) (parent (node (document "d0") (qualified-name "Moments"))))
    (element (id (node (document "d0") (qualified-name "Moments::spaceTimeCoincidentOccurrences"))) (kind "import") (name "spaceTimeCoincidentOccurrences") (declared-name "spaceTimeCoincidentOccurrences") (range (start (line 33) (character 4)) (end (line 33) (character 62))) (parent (node (document "d0") (qualified-name "Moments"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrence::spaceTimeCoincidentOccurrences") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 33) (character 19)) (end (line 33) (character 61))))))
    (element (id (node (document "d0") (qualified-name "Moments::universalEternity"))) (kind "feature decl") (name "universalEternity") (declared-name "universalEternity") (range (start (line 17) (character 4)) (end (line 17) (character 54))) (parent (node (document "d0") (qualified-name "Moments"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Moments::Life"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Life") (range (start (line 1) (character 19)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Moments::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 2) (character 19)) (end (line 2) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Moments::spaceTimeCoincidentOccurrences"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrence::spaceTimeCoincidentOccurrences") (range (start (line 33) (character 19)) (end (line 33) (character 61))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
