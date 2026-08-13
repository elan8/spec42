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
  (document "memory://snapshot/moments.md"
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 4) (end 10 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 12 4) (end 15 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 4) (end 15 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 17 4) (end 17 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 4) (end 17 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 4) (end 22 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 24 4) (end 27 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 4) (end 27 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 4) (end 31 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 19) (end 33 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 35 4) (end 37 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 35 4) (end 37 72))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:28f9af547ca59cf4b9478dbcc6ea545738b660f90c54caeaf91f4b01860c5101") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/moments.md") (qualified-name "Moments"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Life") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrence::spaceTimeCoincidentOccurrences") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrence::spaceTimeCoincidentOccurrences")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/moments.md") (range (start 1 19) (end 1 36)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 2 19) (end 2 42)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 33 19) (end 33 61)) (probe (position 33 19))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrence::spaceTimeCoincidentOccurrences")
      (outcome (status unresolved)))
  )
)
~~~
