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
# SMG
~~~
(model
  (namespace
    (package 'ExtendedOccurrences'
      (class_def 'Interval')
      (class_def 'Moment' :> 'ExtendedOccurrences::Interval'[class_def])
      (class_def 'Timeslice'
        (feature_def 'interval' : 'ExtendedOccurrences::Interval'[class_def])
        (feature_def :>> 'self'[unresolved] : 'ExtendedOccurrences::Timeslice'[class_def]))
      (class_def 'Snapshot' :> 'ExtendedOccurrences::Timeslice'[class_def]
        (feature_def 'moment' :>> 'ExtendedOccurrences::Timeslice::interval'[feature_def] : 'ExtendedOccurrences::Moment'[class_def])
        (feature_def :>> 'self'[unresolved] : 'ExtendedOccurrences::Snapshot'[class_def]))
      (class_def 'Life' :> 'ExtendedOccurrences::Timeslice'[class_def])
      (class_def 'ExtendedOccurrence' :> 'ExtendedOccurrences::Life'[class_def]
        (feature_def :>> 'timeSlices'[unresolved] : 'ExtendedOccurrences::Timeslice'[class_def]
          (multiplicity_range [1..*]))
        (feature_def :>> 'snapshots'[unresolved] :> 'timeSlices'[unresolved] : 'ExtendedOccurrences::Snapshot'[class_def]
          (multiplicity_range [1..*]))
        (expression_def 'at'
          (feature_def :>> 'that'[unresolved] : 'ExtendedOccurrences::Timeslice'[class_def])
          (feature_def in 'interval' : 'ExtendedOccurrences::Interval'[class_def])
          (return_parameter_membership
            (feature_def out 'result' : 'ExtendedOccurrences::Timeslice'[class_def]))
          (binding_connector_def
            (connector_end 'result.portionOf')
            (connector_end 'that'))
          (binding_connector_def
            (connector_end 'result.interval')
            (connector_end 'interval')))
        (expression_def 'while'
          (occurrence_usage in : 'ExtendedOccurrences::Timeslice'[class_def])
          (return_parameter_membership
            (feature_def out 'result' : 'ExtendedOccurrences::Timeslice'[class_def]
              (feature_value (=)))))
        (feature_def 'activeOccurrences' :> 'Occurrences::occurrences'[unresolved]
          (connector_def : 'Occurrences::HappensDuring'[unresolved]
            (connector_end 'that')
            (connector_end 'self')))
        (feature_def 'activeSuboccurrences' :> 'Occurrences::occurrences'[unresolved]
          (connector_def : 'Occurrences::HappensDuring'[unresolved]
            (connector_end 'that')
            (connector_end 'self')))
        (feature_def 'activePerformances' :> 'Performances::performances'[unresolved]
          (connector_def : 'Occurrences::HappensDuring'[unresolved]
            (connector_end 'that')
            (connector_end 'self'))))
      (structure_def 'ExtendedObject' :> 'ExtendedOccurrences::ExtendedOccurrence'[class_def]
        (feature_def 'self' : 'ExtendedOccurrences::ExtendedObject'[structure_def] :>> 'Objects::Object::self'[unresolved] :>> 'ExtendedOccurrence::self'[unresolved])))))
~~~
