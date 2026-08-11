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
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute"))) (kind "package") (name "TimeVaryingAttribute") (declared-name "TimeVaryingAttribute") (range (start (line 0) (character 0)) (end (line 0) (character 1397))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (kind "item def") (name "PwrCmd") (declared-name "PwrCmd") (range (start (line 3) (character 4)) (end (line 3) (character 78))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind "attribute") (name "pwrLevel") (declared-name "pwrLevel") (range (start (line 4) (character 8)) (end (line 4) (character 50))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (kind "part def") (name "Transport2") (declared-name "Transport2") (range (start (line 7) (character 4)) (end (line 7) (character 1249))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (kind "occurrence") (name "") (range (start (line 15) (character 18)) (end (line 15) (character 302))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "portionOfLife") (range (start (line 15) (character 22)) (end (line 15) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (kind "occurrence") (name "") (range (start (line 16) (character 21)) (end (line 16) (character 127))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start") (range (start (line 16) (character 25)) (end (line 16) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (kind "occurrence") (name "") (range (start (line 20) (character 21)) (end (line 20) (character 126))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done") (range (start (line 20) (character 25)) (end (line 20) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (range (start (line 21) (character 16)) (end (line 21) (character 39))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "elapseTime") (range (start (line 21) (character 16)) (end (line 21) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (range (start (line 17) (character 16)) (end (line 17) (character 39))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "elapseTime") (range (start (line 17) (character 16)) (end (line 17) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))) (kind "attribute") (name "currentTime") (declared-name "currentTime") (range (start (line 11) (character 8)) (end (line 11) (character 70))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "localClock.currentTime") (range (start (line 11) (character 22)) (end (line 11) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (range (start (line 10) (character 8)) (end (line 10) (character 46))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::duration") (range (start (line 10) (character 32)) (end (line 10) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (kind "attribute") (name "startTime") (declared-name "startTime") (range (start (line 9) (character 8)) (end (line 9) (character 44))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod"))) (kind "occurrence") (name "transportPeriod") (declared-name "transportPeriod") (range (start (line 33) (character 18)) (end (line 33) (character 270))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (kind "occurrence") (name "") (range (start (line 34) (character 21)) (end (line 34) (character 85))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start") (range (start (line 34) (character 25)) (end (line 34) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (kind "attribute") (name "elapseTime") (declared-name "elapseTime") (range (start (line 35) (character 16)) (end (line 35) (character 39))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "elapseTime") (range (start (line 35) (character 16)) (end (line 35) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingAttribute::s"))) (kind "import") (name "s") (declared-name "s") (range (start (line 1) (character 4)) (end (line 1) (character 25))) (parent (node (document "d0") (qualified-name "TimeVaryingAttribute"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::s") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 24))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (kind redefinition) (ordinal 0)) (authored-target "portionOfLife") (range (start (line 15) (character 22)) (end (line 15) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (kind redefinition) (ordinal 0)) (authored-target "start") (range (start (line 16) (character 25)) (end (line 16) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "done") (range (start (line 20) (character 25)) (end (line 20) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (kind redefinition) (ordinal 0)) (authored-target "elapseTime") (range (start (line 21) (character 16)) (end (line 21) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime")))))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (kind redefinition) (ordinal 0)) (authored-target "elapseTime") (range (start (line 17) (character 16)) (end (line 17) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime")))))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))) (kind redefinition) (ordinal 0)) (authored-target "localClock.currentTime") (range (start (line 11) (character 22)) (end (line 11) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::duration") (range (start (line 10) (character 32)) (end (line 10) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (kind redefinition) (ordinal 0)) (authored-target "start") (range (start (line 34) (character 25)) (end (line 34) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (kind redefinition) (ordinal 0)) (authored-target "elapseTime") (range (start (line 35) (character 16)) (end (line 35) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime")))))
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingAttribute::s"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::s") (range (start (line 1) (character 19)) (end (line 1) (character 24))) (outcome (status unresolved)))
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
