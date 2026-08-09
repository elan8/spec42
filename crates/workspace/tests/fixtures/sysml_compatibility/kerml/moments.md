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

    class UniversalEternity[1] specializes Eternity {
        redefines timeSlices : Period;
        //Includes life.
        redefines snapshots : Moment;
    }

    feature universalEternity : UniversalEternity [1];

    class Period {
        //Includes life and snapshots.
        //↓↓ With UE redef, exactly UE timeslices.
        redefines timeSliceOf : UniversalEternity [1];
    }

    class all InstantOccurrence specializes Occurrence {
        // Probly useful elsewhere, eg, to type snapshots.
        redefines snapshots [1];
        // Or startShot subsets endShot;
    }
    // Or middleTimeslice [0];

    class Moment specializes Period, InstantOccurrence {
        //↓↓ With UE redef, exactly UE snapshots.
        redefines snapshotOf : UniversalEternity [1];
    }

    private import Occurrence::spaceTimeCoincidentOccurrences;
    //UE portion "corresponding" to an occurrence.
    feature coincidentUEPortion : Occurrence [1] subsets spaceTimeCoincidentOccurrences, universalEternity.portions featured by Occurrence;
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
(model
  (namespace
    (package 'Moments'
      (membership_import private -> 'Occurrences::Life'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (class_def 'Eternity' :> 'Life'[unresolved]
        (feature_def :>> 'predecessors'[unresolved]
          (multiplicity_range [0]))
        (feature_def :>> 'successors'[unresolved]
          (multiplicity_range [0]))
        (feature_def :>> 'outsideOfOccurrences'[unresolved]
          (multiplicity_range [0])))
      (class_def 'UniversalEternity' :> 'Moments::Eternity'[class_def]
        (multiplicity_range [1])
        (feature_def :>> 'timeSlices'[unresolved] : 'Moments::Period'[class_def])
        (feature_def :>> 'snapshots'[unresolved] : 'Moments::Moment'[class_def]))
      (feature_def 'universalEternity' : 'Moments::UniversalEternity'[class_def]
        (multiplicity_range [1]))
      (class_def 'Period'
        (feature_def :>> 'timeSliceOf'[unresolved] : 'Moments::UniversalEternity'[class_def]
          (multiplicity_range [1])))
      (class_def sufficient 'InstantOccurrence' :> 'Occurrence'[unresolved]
        (feature_def :>> 'snapshots'[unresolved]
          (multiplicity_range [1])))
      (class_def 'Moment' :> 'Moments::Period'[class_def] :> 'Moments::InstantOccurrence'[class_def]
        (feature_def :>> 'snapshotOf'[unresolved] : 'Moments::UniversalEternity'[class_def]
          (multiplicity_range [1])))
      (membership_import private -> 'Occurrence::spaceTimeCoincidentOccurrences'[unresolved])
      (feature_def 'coincidentUEPortion' : 'Occurrence'[unresolved] :> 'spaceTimeCoincidentOccurrences'[unresolved] :> 'universalEternity::portions'[unresolved]
        (multiplicity_range [1])))))
~~~
