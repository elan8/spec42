# META
~~~ini
description=KerML Variable Feature: TimeVaryingFeatures
type=file
~~~
# SOURCE
~~~kerml
package TimeVaryingFeatures {
    class CC0 {
        var feature x;
        
        portion :>> startShot {
            var feature :>> x = 0;
        }
        
        portion t :> timeSlices {
            var feature y;
            
            portion :>> startShot {
                var feature :>> x = 0; 
                var feature :>> y = 1;
            }
            
            portion t1 :> timeSlices {
                portion :>> startShot {
                    var feature :>> x = 2;
                    var feature :>> y = 3;
                }
            }
        }
    }
    
    class CC1 {
        // var feature x;
        member feature x featured by CC1_snapshots {
            member feature CC1_snapshots :>> Occurrences::Occurrence::snapshots featured by CC1;
        }
        
        // portions are not variable
        portion :>> startShot {
            // var feature :>> x = 0;
            member feature :>> CC1::x featured by CC1_startShot_snapshots = 0 {
                member feature CC1_startShot_snapshots :>> CC1_snapshots featured by CC1::startShot;
            }
        }
        
        portion t :> timeSlices {
            // var feature y;
            member feature y featured by CC1_t_snapshots {
                member feature CC1_t_snapshots :>> Occurrences::Occurrence::snapshots featured by CC1::t;
            }
            portion :>> startShot {
                // var feature :>> x = 0;
                member feature :>> CC1::x featured by CC1_t_startShot_snapshots = 0 {
                    member feature CC1_t_startShot_snapshots :>> CC1_snapshots featured by CC1::t::startShot;
                }
                // var feature :>> y = 1;
                member feature :>> CC1::t::y featured by CC1_t_startShot_snapshots = 1 {
                    member feature CC1_t_startShot_snapshots :>> CC1_t_snapshots featured by CC1::t::startShot;
                }
            }
            portion t1 :> timeSlices {
                portion :>> startShot {
                    // var feature :>> x = 2;
                    member feature :>> CC1::x featured by CC1_t_t1_startShot_snapshots = 2 {
                        member feature CC1_t_t1_startShot_snapshots :>> CC1_snapshots featured by CC1::t::t1::startShot;
                    }
                    // var feature :>> y = 3;
                    member feature :>> CC1::t::y featured by CC1_t_t1_startShot_snapshots = 3 {
                        member feature CC1_t_t1_startShot_snapshots :>> CC1_t_snapshots featured by CC1::t::t1::startShot;
                    }
                }
            }
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/time_varying_features.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 20) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 21) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 24) (end 11 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 26) (end 16 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 28) (end 17 37))
      )
      (diagnostic
        (severity error)
        (code "recovered_attribute_body_element")
        (source "parser")
        (range (start 27 8) (end 32 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 20) (end 32 29))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 34 12) (end 37 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 21) (end 39 31))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 12) (end 44 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 24) (end 44 33))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 46 16) (end 50 16))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 50 16) (end 53 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 26) (end 54 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 28) (end 55 37))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 57 20) (end 61 20))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 61 20) (end 64 16))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:31ce1f9122cfe39ecac9be5089bf0c3b6303844c0a94995c0500148c4ba4125a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "x")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "x")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "y")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::t1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "x")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "y")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t::t1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::t1"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t::t1"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (state literal) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (state literal) (value (kind integer) (integer 3)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t")))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::t1")))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::t1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t")))
      (subtype (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0")))
      (subtype (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t")))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t::t1")))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t::t1")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/time_varying_features.md") (range (start 4 20) (end 4 29)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 5 28) (end 5 29)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 8 21) (end 8 31)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 11 24) (end 11 33)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 12 32) (end 12 33)) (probe (position 12 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 13 32) (end 13 33)) (probe (position 13 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 16 26) (end 16 36)) (probe (position 16 26))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::t1"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 17 28) (end 17 37)) (probe (position 17 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 18 36) (end 18 37)) (probe (position 18 36))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::x")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 19 36) (end 19 37)) (probe (position 19 36))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC0")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0::t::y")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 32 20) (end 32 29)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 39 21) (end 39 31)) (probe (position 39 21))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 44 24) (end 44 33)) (probe (position 44 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 54 26) (end 54 36)) (probe (position 54 26))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1::t::t1"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features.md") (range (start 55 28) (end 55 37)) (probe (position 55 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features.md") (path (named (kind package) (name "TimeVaryingFeatures")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
)
~~~
