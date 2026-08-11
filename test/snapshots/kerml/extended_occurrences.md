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
# NAVIGATION
~~~sexpr
(navigation
)
~~~
