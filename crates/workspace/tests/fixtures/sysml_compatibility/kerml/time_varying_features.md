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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwVar,KwFeature,Ident,Semicolon,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,Ident,Semicolon,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwClass,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
LineComment,
KwPortion,ColonGtGt,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Eq,DecimalValue,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPortion,ColonGtGt,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Eq,DecimalValue,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Eq,DecimalValue,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
KwPortion,ColonGtGt,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Eq,DecimalValue,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Eq,DecimalValue,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TimeVaryingFeatures'
    (class_def 'CC0'
      (feature_def var 'x')
      (feature_def portion :>> 'startShot'
        (feature_def var :>> 'x' value))
      (feature_def portion 't' :> 'timeSlices'
        (feature_def var 'y')
        (feature_def portion :>> 'startShot'
          (feature_def var :>> 'x' value)
          (feature_def var :>> 'y' value))
        (feature_def portion 't1' :> 'timeSlices'
          (feature_def portion :>> 'startShot'
            (feature_def var :>> 'x' value)
            (feature_def var :>> 'y' value)))))
    (class_def 'CC1'
      (line_comment)
      (feature_def member 'x' featured by 'CC1_snapshots'
        (feature_def member 'CC1_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'CC1'))
      (line_comment)
      (feature_def portion :>> 'startShot'
        (line_comment)
        (feature_def member :>> 'CC1::x' value featured by 'CC1_startShot_snapshots'
          (feature_def member 'CC1_startShot_snapshots' :>> 'CC1_snapshots' featured by 'CC1::startShot')))
      (feature_def portion 't' :> 'timeSlices'
        (line_comment)
        (feature_def member 'y' featured by 'CC1_t_snapshots'
          (feature_def member 'CC1_t_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'CC1::t'))
        (feature_def portion :>> 'startShot'
          (line_comment)
          (feature_def member :>> 'CC1::x' value featured by 'CC1_t_startShot_snapshots'
            (feature_def member 'CC1_t_startShot_snapshots' :>> 'CC1_snapshots' featured by 'CC1::t::startShot'))
          (line_comment)
          (feature_def member :>> 'CC1::t::y' value featured by 'CC1_t_startShot_snapshots'
            (feature_def member 'CC1_t_startShot_snapshots' :>> 'CC1_t_snapshots' featured by 'CC1::t::startShot')))
        (feature_def portion 't1' :> 'timeSlices'
          (feature_def portion :>> 'startShot'
            (line_comment)
            (feature_def member :>> 'CC1::x' value featured by 'CC1_t_t1_startShot_snapshots'
              (feature_def member 'CC1_t_t1_startShot_snapshots' :>> 'CC1_snapshots' featured by 'CC1::t::t1::startShot'))
            (line_comment)
            (feature_def member :>> 'CC1::t::y' value featured by 'CC1_t_t1_startShot_snapshots'
              (feature_def member 'CC1_t_t1_startShot_snapshots' :>> 'CC1_t_snapshots' featured by 'CC1::t::t1::startShot'))))))))
~~~
# FORMAT
~~~sysml
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
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_startShot_snapshots'
semantic.unresolved_name 'CC1::startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'CC1_t_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_t_startShot_snapshots'
semantic.unresolved_name 'CC1::t::startShot'
semantic.unresolved_name 'CC1_t_startShot_snapshots'
semantic.unresolved_name 'CC1::t::startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_t_t1_startShot_snapshots'
semantic.unresolved_name 'CC1::t::t1::startShot'
semantic.unresolved_name 'CC1_t_t1_startShot_snapshots'
semantic.unresolved_name 'CC1::t::t1::startShot'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_startShot_snapshots'
semantic.unresolved_name 'CC1::startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'CC1_t_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_t_startShot_snapshots'
semantic.unresolved_name 'CC1::t::startShot'
semantic.unresolved_name 'CC1_t_startShot_snapshots'
semantic.unresolved_name 'CC1::t::startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'CC1_t_t1_startShot_snapshots'
semantic.unresolved_name 'CC1::t::t1::startShot'
semantic.unresolved_name 'CC1_t_t1_startShot_snapshots'
semantic.unresolved_name 'CC1::t::t1::startShot'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TimeVaryingFeatures"))) (name "TimeVaryingFeatures") (declared-name "TimeVaryingFeatures")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingFeatures::CC0"))) (name "CC0") (declared-name "CC0"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingFeatures::CC1"))) (name "CC1") (declared-name "CC1"))
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
