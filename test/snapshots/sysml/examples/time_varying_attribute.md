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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 30) (end 9 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 37) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 32) (end 10 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 22) (end 11 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 22) (end 15 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 25) (end 16 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 17 33) (end 17 38))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 25) (end 20 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 21 33) (end 21 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 25) (end 34 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 35 33) (end 35 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:8441e70e35ca932c47fde0f3ee386edd4044d2315b972d814c0e5d9db94470ef") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SI::s") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Integer"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Time") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "localClock::currentTime")) (expressionOperand (reference "startTime")) (expressionOperand (reference "elapseTime"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "portionOfLife"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "start"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "done"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "elapseTime"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "elapseTime"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::duration"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PwrCmd"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "start")) (invocationCallee (reference "TimeOf"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "start"))))
    (declaration (id (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "elapseTime"))))
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
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "portionOfLife")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "startTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "done")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::duration")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind expressionOperand) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind invocationCallee) (ordinal 0))
      (authored-target "TimeOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "start")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (value (kind non-constant)))
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
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 15 22) (end 15 35)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "portionOfLife")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 11 47) (end 11 56)) (probe (position 11 47))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "startTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime")))))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 11 59) (end 11 69)) (probe (position 11 59))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 16 25) (end 16 30)) (probe (position 16 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 20 25) (end 20 29)) (probe (position 20 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "done")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 17 20) (end 17 30)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 21 20) (end 21 30)) (probe (position 21 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 10 32) (end 10 45)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::duration")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 13 24) (end 13 30)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::pwrCmd"))) (kind featureTyping) (ordinal 0) (authored-target "PwrCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::PwrCmd")))))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 9 37) (end 9 42)) (probe (position 9 37))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind expressionOperand) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 9 30) (end 9 36)) (probe (position 9 30))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind invocationCallee) (ordinal 0) (authored-target "TimeOf")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 34 25) (end 34 30)) (probe (position 34 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "start")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_attribute.md") (range (start 35 20) (end 35 30)) (probe (position 35 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_attribute.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elapseTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_attribute.md") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime")))))
  )
)
~~~
