# META
~~~ini
description=SysML Example (Timeslice and Snapshot): TimeVaryingAttribute
type=file
~~~
# SOURCE
~~~sysml
package TimeVaryingAttribute {
    private import SI::s;
    
    item def PwrCmd {
        attribute pwrLevel: ScalarValues::Integer;
    }
    
    part def Transport2 {
        private import Time::*;
        attribute startTime = TimeOf(start);
        attribute elapseTime :> ISQ::duration;
        attribute :>> localClock.currentTime = startTime + elapseTime;
        
        out item pwrCmd:PwrCmd;
        // Lifetime conditions
        timeslice :>> portionOfLife {
            snapshot :>> start {
                :>> elapseTime = 0 [s];
                :>> pwrCmd.pwrLevel = 0;
            }
            snapshot :>> done {
                :>> elapseTime = 2 [s];
                :>> pwrCmd.pwrLevel = 1;
            }
        }
        
 //     Alternative:
 //       // initial conditions
 //       :>> portionOfLife.start : C {
 //           :>> elapseTime = 0 [s];
 //           :>> pwrCmd.pwrLevel = 0;
 //       }
 
        timeslice transportPeriod {
            snapshot :>> start{
                :>> elapseTime = 1 [s];
            }
            snapshot :>> done {
                :>> elapseTime = 1.5 [s];
            }
           :>> pwrCmd.pwrLevel = 2*elapseTime.num;
        }
        
//      Alternative:
//        // final conditions
//        :>> portionOfLife.done {
//            :>> elapseTime = 2 [s];
//            :>> pwrCmd.pwrLevel = 1;
//        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/time_varying_attribute.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 28) (end 4 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 23) (end 8 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 10 32) (end 10 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 11 22) (end 11 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 15 8) (end 24 9))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "parser")
        (range (start 18 16) (end 19 12))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 18 16) (end 19 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 33 8) (end 41 9))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8441e70e35ca932c47fde0f3ee386edd4044d2315b972d814c0e5d9db94470ef") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::s") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Time") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock::currentTime"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::duration"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PwrCmd"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SI::s")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Time")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "localClock::currentTime")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::duration")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 1 19) (end 1 24)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SI::s")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 4 28) (end 4 49)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 8 23) (end 8 30)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Time")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 11 22) (end 11 44)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "localClock::currentTime")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 10 32) (end 10 45)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::duration")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 13 24) (end 13 30)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind featureTyping) (ordinal 0) (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd")))))
  )
)
~~~
