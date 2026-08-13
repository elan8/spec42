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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 31) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 18) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 18) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 18) (end 9 38))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 16) (end 21 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 30) (end 21 47))
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 29 37) (end 29 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 16) (end 31 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 29) (end 31 46))
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
    (declaration (id (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Eternity"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Life"))))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "predecessors"))))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "successors"))))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "outsideOfOccurrences"))))
    (declaration (id (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Period")) (specialization (reference "InstantOccurrence"))))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UniversalEternity")) (redefinition (reference "snapshotOf"))))
    (declaration (id (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Period"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UniversalEternity")) (redefinition (reference "timeSliceOf"))))
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
    (reference (id (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Eternity"))) (kind specialization) (ordinal 0))
      (authored-target "Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "predecessors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "successors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "outsideOfOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (kind specialization) (ordinal 0))
      (authored-target "Period")
      (outcome (status resolved) (target (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Period")))))
    (reference (id (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (kind specialization) (ordinal 1))
      (authored-target "InstantOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "UniversalEternity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "snapshotOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "UniversalEternity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "timeSliceOf")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (target (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Period"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (kind specialization) (ordinal 0)))
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
  (query (document "memory://snapshot/moments.md") (range (start 4 31) (end 4 35)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Eternity"))) (kind specialization) (ordinal 0) (authored-target "Life")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 7 18) (end 7 30)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "predecessors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 8 18) (end 8 28)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "successors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 9 18) (end 9 38)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "outsideOfOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 29 29) (end 29 35)) (probe (position 29 29))
    (reference (id (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (kind specialization) (ordinal 0) (authored-target "Period")
      (outcome (status resolved) (target (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Period")))))
  )
  (query (document "memory://snapshot/moments.md") (range (start 29 37) (end 29 54)) (probe (position 29 37))
    (reference (id (source (node (document "memory://snapshot/moments.md") (qualified-name "Moments::Moment"))) (kind specialization) (ordinal 1) (authored-target "InstantOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 31 29) (end 31 46)) (probe (position 31 29))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "UniversalEternity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 31 16) (end 31 26)) (probe (position 31 16))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "snapshotOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 21 30) (end 21 47)) (probe (position 21 30))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "UniversalEternity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/moments.md") (range (start 21 16) (end 21 27)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/moments.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "timeSliceOf")
      (outcome (status unresolved)))
  )
)
~~~
