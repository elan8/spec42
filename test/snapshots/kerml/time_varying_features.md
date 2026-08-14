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
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 2 8) (end 4 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 8) (end 8 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 8) (end 23 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 27 8) (end 32 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 32 8) (end 39 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 39 8) (end 67 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:31ce1f9122cfe39ecac9be5089bf0c3b6303844c0a94995c0500148c4ba4125a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC0"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features.md") (qualified-name "TimeVaryingFeatures::CC1"))) (kind class-def) (membership (kind owning) (visibility default)))
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
