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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Dot,Ident,Eq,Ident,Plus,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
LineComment,
KwTimeslice,ColonGtGt,Ident,OpenCurly,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Dot,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Dot,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
KwTimeslice,Ident,OpenCurly,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
ColonGtGt,Ident,Dot,Ident,Eq,DecimalValue,Star,Ident,Dot,Ident,Semicolon,
CloseCurly,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TimeVaryingAttribute'
    (import_decl private 'SI::s')
    (item_def 'PwrCmd'
      (attribute_usage 'pwrLevel' : 'ScalarValues::Integer'))
    (part_def 'Transport2'
      (import_decl private 'Time::*')
      (attribute_usage 'startTime' value)
      (attribute_usage 'elapseTime' :> 'ISQ::duration')
      (attribute_usage :>> 'localClock.currentTime' value)
      (item_usage out 'pwrCmd' : 'PwrCmd')
      (line_comment)
      (portion_usage timeslice :>> 'portionOfLife'
        (portion_usage snapshot :>> 'start'
          (default_ref_usage :>> 'elapseTime' value)
          (default_ref_usage :>> 'pwrCmd.pwrLevel' value))
        (portion_usage snapshot :>> 'done'
          (default_ref_usage :>> 'elapseTime' value)
          (default_ref_usage :>> 'pwrCmd.pwrLevel' value)))
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (portion_usage timeslice 'transportPeriod'
        (portion_usage snapshot :>> 'start'
          (default_ref_usage :>> 'elapseTime' value))
        (portion_usage snapshot :>> 'done'
          (default_ref_usage :>> 'elapseTime' value))
        (default_ref_usage :>> 'pwrCmd.pwrLevel' value))
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment))))
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
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ISQ::duration'
semantic.unresolved_name 'localClock::currentTime'
semantic.unresolved_name 'portionOfLife'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Integer'
semantic.unresolved_name 'ISQ::duration'
semantic.unresolved_name 'localClock::currentTime'
semantic.unresolved_name 'portionOfLife'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TimeVaryingAttribute"))) (name "TimeVaryingAttribute") (declared-name "TimeVaryingAttribute")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd"))) (name "PwrCmd") (declared-name "PwrCmd")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd::pwrLevel"))) (name "pwrLevel") (declared-name "pwrLevel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::PwrCmd")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (name "Transport2") (declared-name "Transport2") (declared)
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::"))) (name "") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::"))) (name "") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::::elapseTime"))) (name "elapseTime") (declared-name "elapseTime") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2")))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence"))) (name "") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::::#occurrence::elapseTime"))) (name "elapseTime") (declared-name "elapseTime") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2")))))
                  )
                )
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))) (name "currentTime") (declared-name "currentTime") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "startTime")) (expression (kind "featureReference") (reference "elapseTime")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::currentTime"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::elapseTime"))) (name "elapseTime") (declared-name "elapseTime") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (name "startTime") (declared-name "startTime") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "TimeOf"))) (arguments (argument (expression (kind "featureReference") (reference "start"))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::startTime"))) (role feature-value))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod"))) (name "transportPeriod") (declared-name "transportPeriod") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::"))) (name "") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2::transportPeriod::::elapseTime"))) (name "elapseTime") (declared-name "elapseTime") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "TimeVaryingAttribute::Transport2")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "TimeVaryingAttribute::s"))) (name "s") (declared-name "s"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/time_varying_attribute.md"
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
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 11 8) (end 11 70))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 13 8) (end 13 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 15 18) (end 15 302))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 16 21) (end 16 127))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 17 16) (end 17 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 17 16) (end 17 39))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 20 21) (end 20 126))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 21 16) (end 21 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 21 16) (end 21 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 34 21) (end 34 85))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 35 16) (end 35 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 35 16) (end 35 39))
      )
    )
  )
)
~~~
