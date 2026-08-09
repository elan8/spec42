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
        attribute pwrLevel : ScalarValues::Integer;
    }

    part def Transport2 {
        private import Time::*;
        attribute startTime = TimeOf(start);
        attribute elapseTime :> ISQ::duration;
        attribute :>> localClock.currentTime = startTime + elapseTime;

        out item pwrCmd : PwrCmd;
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
            snapshot :>> start {
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
(model
  (namespace
    (package 'TimeVaryingAttribute'
      (membership_import private -> 'SI::s'[unresolved])
      (item_def 'PwrCmd'
        (attribute_usage composite 'pwrLevel' : 'ScalarValues::Integer'[unresolved]))
      (part_def 'Transport2'
        (namespace_import private -> 'Time'[unresolved])
        (attribute_usage composite 'startTime'
          (feature_value (=)))
        (attribute_usage composite 'elapseTime' :> 'ISQ::duration'[unresolved])
        (attribute_usage composite :>> 'localClock::currentTime'[unresolved]
          (feature_value (=)))
        (item_usage out 'pwrCmd' : 'TimeVaryingAttribute::PwrCmd'[item_def])
        (occurrence_usage composite :>> 'portionOfLife'[unresolved]
          (occurrence_usage composite :>> 'start'[unresolved]
            (reference_usage reference :>> 'TimeVaryingAttribute::Transport2::elapseTime'[attribute_usage]
              (feature_value (=)))
            (reference_usage reference :>> 'TimeVaryingAttribute::PwrCmd::pwrLevel'[attribute_usage]
              (feature_value (=))))
          (occurrence_usage composite :>> 'done'[unresolved]
            (reference_usage reference :>> 'TimeVaryingAttribute::Transport2::elapseTime'[attribute_usage]
              (feature_value (=)))
            (reference_usage reference :>> 'TimeVaryingAttribute::PwrCmd::pwrLevel'[attribute_usage]
              (feature_value (=)))))
        (occurrence_usage composite 'transportPeriod'
          (occurrence_usage composite :>> 'start'[unresolved]
            (reference_usage reference :>> 'TimeVaryingAttribute::Transport2::elapseTime'[attribute_usage]
              (feature_value (=))))
          (occurrence_usage composite :>> 'done'[unresolved]
            (reference_usage reference :>> 'TimeVaryingAttribute::Transport2::elapseTime'[attribute_usage]
              (feature_value (=))))
          (reference_usage reference :>> 'TimeVaryingAttribute::PwrCmd::pwrLevel'[attribute_usage]
            (feature_value (=))))))))
~~~
