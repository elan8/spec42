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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
LineComment,
LineComment,
KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwClass,Ident,OpenSquare,DecimalValue,CloseSquare,KwSpecializes,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,Semicolon,LineComment,
KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwClass,Ident,OpenCurly,LineComment,
LineComment,
KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwClass,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
LineComment,
KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,LineComment,
KwClass,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
LineComment,
KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,
Ident,Dot,Ident,
KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Moments'
    (import_decl private 'Occurrences::Life')
    (import_decl private 'Occurrences::Occurrence')
    (class_def 'Eternity' :> 'Life'
      (line_comment)
      (line_comment)
      (feature_def :>> 'predecessors' multiplicity)
      (feature_def :>> 'successors' multiplicity)
      (feature_def :>> 'outsideOfOccurrences' multiplicity))
    (class_def 'UniversalEternity' multiplicity     (multiplicity_range) :> 'Eternity'
      (feature_def :>> 'timeSlices' : 'Period')
      (line_comment)
      (feature_def :>> 'snapshots' : 'Moment'))
    (feature_def 'universalEternity' : 'UniversalEternity' multiplicity)
    (class_def 'Period'
      (line_comment)
      (line_comment)
      (feature_def :>> 'timeSliceOf' : 'UniversalEternity' multiplicity))
    (class_def all 'InstantOccurrence' :> 'Occurrence'
      (line_comment)
      (feature_def :>> 'snapshots' multiplicity)
      (line_comment))
    (line_comment)
    (class_def 'Moment' :> 'Period', 'InstantOccurrence'
      (line_comment)
      (feature_def :>> 'snapshotOf' : 'UniversalEternity' multiplicity))
    (import_decl private 'Occurrence::spaceTimeCoincidentOccurrences')
    (line_comment)
    (feature_def 'coincidentUEPortion' : 'Occurrence' multiplicity :> 'spaceTimeCoincidentOccurrences', 'universalEternity.portions' featured by 'Occurrence')))
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
# EXPECTED
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'predecessors'
semantic.unresolved_name 'successors'
semantic.unresolved_name 'outsideOfOccurrences'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'timeSliceOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'snapshotOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'spaceTimeCoincidentOccurrences'
semantic.unresolved_name 'universalEternity::portions'
semantic.unresolved_name 'Occurrence'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Life'
semantic.unresolved_name 'predecessors'
semantic.unresolved_name 'successors'
semantic.unresolved_name 'outsideOfOccurrences'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'timeSliceOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'snapshotOf'
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'spaceTimeCoincidentOccurrences'
semantic.unresolved_name 'universalEternity::portions'
semantic.unresolved_name 'Occurrence'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Moments"))) (name "Moments") (declared-name "Moments")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Moments::Eternity"))) (name "Eternity") (declared-name "Eternity"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Moments::Life"))) (name "Life") (declared-name "Life"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Moments::Moment"))) (name "Moment") (declared-name "Moment"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Moments::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Moments::Period"))) (name "Period") (declared-name "Period"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Moments::UniversalEternity"))) (name "UniversalEternity") (declared-name "UniversalEternity"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Moments::all"))) (name "all") (declared-name "all"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Moments::coincidentUEPortion"))) (name "coincidentUEPortion") (declared-name "coincidentUEPortion"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Moments::spaceTimeCoincidentOccurrences"))) (name "spaceTimeCoincidentOccurrences") (declared-name "spaceTimeCoincidentOccurrences"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Moments::universalEternity"))) (name "universalEternity") (declared-name "universalEternity"))
      )
    )
  )
  (relationships
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
  (document "kerml/moments.md"
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
