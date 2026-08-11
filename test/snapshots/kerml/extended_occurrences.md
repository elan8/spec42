# META
~~~ini
description=KerML Enhancements: ExtendedOccurrences
type=file
~~~
# SOURCE
~~~kerml
package ExtendedOccurrences {
    class Interval;
    class Moment :> Interval;
    class Timeslice {
        feature interval : Interval;
        :>> self : Timeslice;
    }
    class Snapshot :> Timeslice {
        feature moment :>> interval : Moment;
        :>> self : Snapshot;
    }
    class Life :> Timeslice;
    class ExtendedOccurrence :> Life {
        :>> timeSlices : Timeslice [1..*];
        :>> snapshots :> timeSlices : Snapshot [1..*];
        expr at {
        	:>> that : Timeslice;
            in interval : Interval;
            return result : Timeslice;

            binding result.portionOf = that;
            binding result.interval = interval;
        }

        expr while {
            in timeslice : Timeslice;
            return result : Timeslice = at(timeslice.interval);
        }
        
        var feature activeOccurrences :> Occurrences::occurrences {
        	connector : Occurrences::HappensDuring from [1] that to [1] self;
        }
        
        var feature activeSuboccurrences :> Occurrences::occurrences {
        	connector : Occurrences::HappensDuring from [1] that to [1] self;
        }
        
        // occurrences and performances are abstract package-level features.
        // It would be nice to put the variable next to them, but they cannot 
        // be package-level, or featured by Anything. Nevertheless, since
        // Occurrence is a specialization of Anything, it will have these 
        // features (might be worth redefining them explicitly), so the
        // variables can subset them. In the case below, performances will
        // contain every step in the occurrence, which is the correct domain
        // for the variable.
        var feature activePerformances :> Performances::performances {
        	connector : Occurrences::HappensDuring from [1] that to [1] self;
        }
    }
    struct ExtendedObject :> ExtendedOccurrence {
        feature self : ExtendedObject :>> Objects::Object::self, ExtendedOccurrence::self;
    }

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "extended_occurrences.md"
    (diagnostics
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,Semicolon,
KwClass,Ident,ColonGt,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,ColonGt,Ident,OpenCurly,
KwFeature,Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,ColonGt,Ident,Semicolon,
KwClass,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
ColonGtGt,Ident,ColonGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwExpr,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
KwBinding,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBinding,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwExpr,KwWhile,OpenCurly,
KwIn,KwTimeslice,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,OpenParen,KwTimeslice,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwVar,KwFeature,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwConnector,Colon,Ident,ColonColon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwVar,KwFeature,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwConnector,Colon,Ident,ColonColon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
KwVar,KwFeature,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwConnector,Colon,Ident,ColonColon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwStruct,Ident,ColonGt,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ExtendedOccurrences'
    (class_def 'Interval')
    (class_def 'Moment' :> 'Interval')
    (class_def 'Timeslice'
      (feature_def 'interval' : 'Interval')
      (feature_def :>> 'self' : 'Timeslice'))
    (class_def 'Snapshot' :> 'Timeslice'
      (feature_def 'moment' :>> 'interval' : 'Moment')
      (feature_def :>> 'self' : 'Snapshot'))
    (class_def 'Life' :> 'Timeslice')
    (class_def 'ExtendedOccurrence' :> 'Life'
      (feature_def :>> 'timeSlices' : 'Timeslice' multiplicity)
      (feature_def :>> 'snapshots' :> 'timeSlices' : 'Snapshot' multiplicity)
      (expression_def
        (feature_def :>> 'that' : 'Timeslice')
        (feature_def in 'interval' : 'Interval')
        (return_member)
        (binding_connector
          (connector_end)
          (connector_end))
        (binding_connector
          (connector_end)
          (connector_end)))
      (expression_def
        (portion_usage in timeslice : 'Timeslice')
        (return_member))
      (feature_def var 'activeOccurrences' :> 'Occurrences::occurrences'
        (connector_def : 'Occurrences::HappensDuring'
          (connector_end)
          (connector_end)))
      (feature_def var 'activeSuboccurrences' :> 'Occurrences::occurrences'
        (connector_def : 'Occurrences::HappensDuring'
          (connector_end)
          (connector_end)))
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def var 'activePerformances' :> 'Performances::performances'
        (connector_def : 'Occurrences::HappensDuring'
          (connector_end)
          (connector_end))))
    (structure_def 'ExtendedObject' :> 'ExtendedOccurrence'
      (feature_def 'self' : 'ExtendedObject' :>> 'Objects::Object::self', 'ExtendedOccurrence::self'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'self'
semantic.unresolved_name 'self'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Occurrences::occurrences'
semantic.unresolved_name 'Occurrences::HappensDuring'
semantic.unresolved_name 'Occurrences::occurrences'
semantic.unresolved_name 'Occurrences::HappensDuring'
semantic.unresolved_name 'Performances::performances'
semantic.unresolved_name 'Occurrences::HappensDuring'
semantic.unresolved_name 'Objects::Object::self'
semantic.unresolved_name 'ExtendedOccurrence::self'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'self'
semantic.unresolved_name 'self'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'snapshots'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Occurrences::occurrences'
semantic.unresolved_name 'Occurrences::HappensDuring'
semantic.unresolved_name 'Occurrences::occurrences'
semantic.unresolved_name 'Occurrences::HappensDuring'
semantic.unresolved_name 'Performances::performances'
semantic.unresolved_name 'Occurrences::HappensDuring'
semantic.unresolved_name 'Objects::Object::self'
semantic.unresolved_name 'ExtendedOccurrence::self'
~~~
# FORMAT
~~~sysml
package ExtendedOccurrences {
    class Interval;
    class Moment :> Interval;
    class Timeslice {
        feature interval : Interval;
        :>> self : Timeslice;
    }
    class Snapshot :> Timeslice {
        feature moment :>> interval : Moment;
        :>> self : Snapshot;
    }
    class Life :> Timeslice;
    class ExtendedOccurrence :> Life {
        :>> timeSlices : Timeslice [1..*];
        :>> snapshots :> timeSlices : Snapshot [1..*];
        expr at {
        	:>> that : Timeslice;
            in interval : Interval;
            return result : Timeslice;

            binding result.portionOf = that;
            binding result.interval = interval;
        }

        expr while {
            in timeslice : Timeslice;
            return result : Timeslice = at(timeslice.interval);
        }
        
        var feature activeOccurrences :> Occurrences::occurrences {
        	connector : Occurrences::HappensDuring from [1] that to [1] self;
        }
        
        var feature activeSuboccurrences :> Occurrences::occurrences {
        	connector : Occurrences::HappensDuring from [1] that to [1] self;
        }
        
        // occurrences and performances are abstract package-level features.
        // It would be nice to put the variable next to them, but they cannot 
        // be package-level, or featured by Anything. Nevertheless, since
        // Occurrence is a specialization of Anything, it will have these 
        // features (might be worth redefining them explicitly), so the
        // variables can subset them. In the case below, performances will
        // contain every step in the occurrence, which is the correct domain
        // for the variable.
        var feature activePerformances :> Performances::performances {
        	connector : Occurrences::HappensDuring from [1] that to [1] self;
        }
    }
    struct ExtendedObject :> ExtendedOccurrence {
        feature self : ExtendedObject :>> Objects::Object::self, ExtendedOccurrence::self;
    }

}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a485154465639c6a7defd7178f21a4f333d44c73322ea2c9768abe1302df8091") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences"))) (kind "package") (name "ExtendedOccurrences") (declared-name "ExtendedOccurrences") (range (start (line 0) (character 0)) (end (line 0) (character 2023))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (kind "classifier decl") (name "ExtendedObject") (declared-name "ExtendedObject") (range (start (line 49) (character 4)) (end (line 49) (character 146))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (kind "classifier decl") (name "ExtendedOccurrence") (declared-name "ExtendedOccurrence") (range (start (line 12) (character 4)) (end (line 12) (character 1554))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::Interval"))) (kind "classifier decl") (name "Interval") (declared-name "Interval") (range (start (line 1) (character 4)) (end (line 1) (character 19))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::Life"))) (kind "classifier decl") (name "Life") (declared-name "Life") (range (start (line 11) (character 4)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::Moment"))) (kind "classifier decl") (name "Moment") (declared-name "Moment") (range (start (line 2) (character 4)) (end (line 2) (character 29))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::Snapshot"))) (kind "classifier decl") (name "Snapshot") (declared-name "Snapshot") (range (start (line 7) (character 4)) (end (line 7) (character 114))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
    (element (id (node (document "d0") (qualified-name "ExtendedOccurrences::Timeslice"))) (kind "classifier decl") (name "Timeslice") (declared-name "Timeslice") (range (start (line 3) (character 4)) (end (line 3) (character 94))) (parent (node (document "d0") (qualified-name "ExtendedOccurrences"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
