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
  (document "memory://snapshot/extended_occurrences.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 8) (end 5 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 12) (end 5 16))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 8) (end 9 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 12) (end 9 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 12) (end 13 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 14 8) (end 14 54))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 15 8) (end 24 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 24 8) (end 29 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 29 8) (end 33 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 33 8) (end 45 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 45 8) (end 48 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 42) (end 50 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 65) (end 50 89))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3d83147000b14eef7f50c10362c18c6e228d220c5fef96c7c57258ca5abdeb93") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ExtendedObject")) (redefinition (reference "Objects::Object::self")) (redefinition (reference "ExtendedOccurrence::self")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Life")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Timeslice")) (redefinition (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Interval"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Timeslice")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Moment"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Interval")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Timeslice")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Snapshot")) (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Timeslice")) (redefinition (reference "self")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "ExtendedObject")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Objects::Object::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind redefinition) (ordinal 1))
      (authored-target "ExtendedOccurrence::self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (kind specialization) (ordinal 0))
      (authored-target "Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life"))) (kind specialization) (ordinal 0))
      (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Moment"))) (kind specialization) (ordinal 0))
      (authored-target "Interval")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Interval")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot"))) (kind specialization) (ordinal 0))
      (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Snapshot")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life"))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Moment"))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Interval"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Moment"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot"))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject")))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self")))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject")) (scopes any))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life")) (scopes any))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence")))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life")))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Moment")))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Interval")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot")))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot")) (scopes any))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 49 29) (end 49 47)) (probe (position 49 29))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 50 23) (end 50 37)) (probe (position 50 23))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind featureTyping) (ordinal 0) (authored-target "ExtendedObject")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 50 42) (end 50 63)) (probe (position 50 42))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind redefinition) (ordinal 0) (authored-target "Objects::Object::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 50 65) (end 50 89)) (probe (position 50 65))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedObject::self"))) (kind redefinition) (ordinal 1) (authored-target "ExtendedOccurrence::self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 12 32) (end 12 36)) (probe (position 12 32))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::ExtendedOccurrence"))) (kind specialization) (ordinal 0) (authored-target "Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 13 25) (end 13 34)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 13 12) (end 13 22)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "ExtendedOccurrence")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 11 18) (end 11 27)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Life"))) (kind specialization) (ordinal 0) (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 2 20) (end 2 28)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Moment"))) (kind specialization) (ordinal 0) (authored-target "Interval")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Interval")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 7 22) (end 7 31)) (probe (position 7 22))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot"))) (kind specialization) (ordinal 0) (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 9 19) (end 9 27)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Snapshot")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Snapshot")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 9 12) (end 9 16)) (probe (position 9 12))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Snapshot")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 5 19) (end 5 28)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Timeslice")
      (outcome (status resolved) (target (node (document "memory://snapshot/extended_occurrences.md") (qualified-name "ExtendedOccurrences::Timeslice")))))
    )
  )
  (query (document "memory://snapshot/extended_occurrences.md") (range (start 5 12) (end 5 16)) (probe (position 5 12))
    (reference (id (source (node (document "memory://snapshot/extended_occurrences.md") (path (named (kind package) (name "ExtendedOccurrences")) (named (kind class-def) (name "Timeslice")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
)
~~~
