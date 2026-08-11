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
  (document "time_varying_attribute.md"
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
        (range (start 4 8) (end 4 50))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 9 8) (end 9 44))
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
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 13 8) (end 13 71))
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
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "sysml")
        (range (start 18 16) (end 18 53))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 18 16) (end 18 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 25) (end 20 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 34 25) (end 34 30))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "e5e7e3bbd466078f05bd056dbb047c2f9d5c34e355217966f9df8945ffe92e3d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute"))) (kind "package") (name "TimeVaryingAttribute") (declared-name "TimeVaryingAttribute"))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (kind "item def") (name "PwrCmd") (declared-name "PwrCmd") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind "attribute") (name "pwrLevel") (declared-name "pwrLevel") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (kind "part def") (name "Transport2") (declared-name "Transport2") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "portionOfLife")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "elapseTime")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "elapseTime")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock.currentTime")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::duration")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind "attribute") (name "startTime") (declared-name "startTime") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod"))) (kind "occurrence") (name "transportPeriod") (declared-name "transportPeriod") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "elapseTime")))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::s"))) (kind "import") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "TimeVaryingAttribute"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::s") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (kind redefinition) (ordinal 0)) (authored-target "portionOfLife") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (kind redefinition) (ordinal 0)) (authored-target "start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "done") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (kind redefinition) (ordinal 0)) (authored-target "elapseTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime")))))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (kind redefinition) (ordinal 0)) (authored-target "elapseTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime")))))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "localClock.currentTime") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::duration") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (kind redefinition) (ordinal 0)) (authored-target "start") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (kind redefinition) (ordinal 0)) (authored-target "elapseTime") (outcome (status resolved) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime")))))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::s"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::s") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::startTime")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 20 25) (end 20 29)) (probe (position 20 25))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))
        (kind redefinition) (ordinal 0) (authored-target "done")
        (range (start 20 25) (end 20 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 19) (end 1 24)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::s"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::s")
        (range (start 1 19) (end 1 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 25) (end 16 30)) (probe (position 16 25))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))
        (kind redefinition) (ordinal 0) (authored-target "start")
        (range (start 16 25) (end 16 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 25) (end 34 30)) (probe (position 34 25))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))
        (kind redefinition) (ordinal 0) (authored-target "start")
        (range (start 34 25) (end 34 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 32) (end 10 45)) (probe (position 10 32))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::duration")
        (range (start 10 32) (end 10 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 22) (end 15 35)) (probe (position 15 22))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))
        (kind redefinition) (ordinal 0) (authored-target "portionOfLife")
        (range (start 15 22) (end 15 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 30)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))
        (kind redefinition) (ordinal 0) (authored-target "elapseTime")
        (range (start 17 16) (end 17 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime") (range (start 17 16) (end 17 39)))
        )
      )
    )
    (query (range (start 21 16) (end 21 30)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))
        (kind redefinition) (ordinal 0) (authored-target "elapseTime")
        (range (start 21 16) (end 21 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime") (range (start 21 16) (end 21 39)))
        )
      )
    )
    (query (range (start 35 16) (end 35 30)) (probe (position 35 16))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))
        (kind redefinition) (ordinal 0) (authored-target "elapseTime")
        (range (start 35 16) (end 35 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime") (range (start 35 16) (end 35 39)))
        )
      )
    )
    (query (range (start 11 22) (end 11 44)) (probe (position 11 22))
      (reference
        (source (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))
        (kind redefinition) (ordinal 0) (authored-target "localClock.currentTime")
        (range (start 11 22) (end 11 44))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
