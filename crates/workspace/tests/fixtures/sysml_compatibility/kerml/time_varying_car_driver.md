# META
~~~ini
description=KerML Variable Feature: TimeVaryingCarDriver
type=file
~~~
# SOURCE
~~~kerml
package TimeVaryingCarDriver {
    private import ScalarValues::*;
    
    // Example model without variable features.
    
    struct Person0 {
        feature isLicensed : Boolean [0..1];
    }
    
    struct Car0 {
        feature driver : Person0 [0..1];
        
        portion :>> startShot {
        	feature :>> driver [0];
        }
        
        succession first startShot then operated; 

        portion operated [0..*] :> timeSlices {
            feature :>> driver [1] {
                feature :>> isLicensed = true;
            }
        }

        abstract feature carParts [0..*];
        feature engine [1] :> carParts;
        feature transmission [1] :> carParts;
        
        connector drive from engine to transmission;      
    }
    
    // Example model with "variable features" identified
    
    struct Person1 {
        var feature isLicensed : Boolean [1];
    }
    
    struct Car1 {
        var feature driver : Person1 [0..1];
        
        portion :>> startShot {
        	var feature :>> driver [0]; 
        }
        
        succession first startShot then operated; 

        portion operated [0..*] :> timeSlices {
            var feature :>> driver [1] {
                var feature :>> isLicensed = true;
            }
        }
        
        abstract var feature carParts [0..*];
        var feature engine [1] :> carParts;
        var feature transmission [1] :> carParts;
        
        var connector drive from engine to transmission;
    }
    
  	// Semantic equivalent of implied relationships for variable features in
  	// the previous model
  
    struct Person1_ {
        // var feature isLicensed : Boolean [1];
        member feature isLicensed : Boolean [1] featured by Person_snapshots {
            member feature Person_snapshots :>> Occurrences::Occurrence::snapshots featured by Person1_;
        }
        member feature name : String [1] featured by Person_snapshots {
            member feature Person_snapshots :>> Occurrences::Occurrence::snapshots featured by Person1_;
        }
    }
    
    struct Car1_ {
        // var feature driver : Person [0..1];
        member feature driver : Person1_ [0..1] featured by Car_snapshots {
            member feature Car_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }
        
        portion :>> startShot {
        	// var feature :>> driver [0];
           	member feature :>> Car1_::driver [0] featured by Car_startShot_snapshots {
                member feature Car_startShot_snapshots :>> Car_snapshots featured by Car1_::startShot;
        	}
        }
        
        succession first startShot then operated; 

        portion operated [0..*] :> timeSlices {
            // var feature :>> driver [1]
            member feature :>> Car1_::driver [1] featured by Car_operated_snapshots {
                member feature Car_operated_snapshots :>> Car_snapshots featured by Car1_::operated;
                // var feature :>> isLicensed = true;
                member feature isLicensed1 :>> Person1_::isLicensed featured by Car_operated_driver_snapshots = true {
                    member feature Car_operated_driver_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_::operated::driver;
                }
            }
        }

        // var abstract feature carParts [0..*];
        member abstract feature carParts [0..*] featured by Car_snapshots {
            member feature Car_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }
        
        // var feature engine [1];
        member feature engine [1] :> carParts featured by Car_snapshots1 {
            member feature Car_snapshots1 :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }       
        
        // var feature transmission [1];
        member feature transmission [1] :> carParts featured by Car_snapshots1 {
            member feature Car_snapshots1 :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }       
        
        // var connector drive from engine to transmission;
        member connector drive featured by Car_snapshots from engine to transmission {
            member feature Car_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }
    }
    
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
KwStruct,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwStruct,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwSuccession,KwFirst,Ident,KwThen,Ident,Semicolon,
KwPortion,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwFeature,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwConnector,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
CloseCurly,
LineComment,
KwStruct,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwStruct,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwSuccession,KwFirst,Ident,KwThen,Ident,Semicolon,
KwPortion,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwVar,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwVar,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwVar,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwVar,KwConnector,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
CloseCurly,
LineComment,
LineComment,
KwStruct,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwStruct,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
KwPortion,ColonGtGt,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwSuccession,KwFirst,Ident,KwThen,Ident,Semicolon,
KwPortion,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
LineComment,
KwMember,KwFeature,ColonGtGt,Ident,ColonColon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Eq,KwTrue,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,ColonColon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
LineComment,
KwMember,KwAbstract,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
LineComment,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
LineComment,
KwMember,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,KwFeatured,KwBy,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
LineComment,
KwMember,KwConnector,Ident,KwFeatured,KwBy,Ident,KwFrom,Ident,KwTo,Ident,OpenCurly,
KwMember,KwFeature,Ident,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,KwFeatured,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TimeVaryingCarDriver'
    (import_decl private 'ScalarValues::*')
    (line_comment)
    (structure_def 'Person0'
      (feature_def 'isLicensed' : 'Boolean' multiplicity))
    (structure_def 'Car0'
      (feature_def 'driver' : 'Person0' multiplicity)
      (feature_def portion :>> 'startShot'
        (feature_def :>> 'driver' multiplicity))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (feature_def portion 'operated' multiplicity :> 'timeSlices'
        (feature_def :>> 'driver' multiplicity
          (feature_def :>> 'isLicensed' value)))
      (feature_def abstract 'carParts' multiplicity)
      (feature_def 'engine' multiplicity :> 'carParts')
      (feature_def 'transmission' multiplicity :> 'carParts')
      (connector_def 'drive'
        (connector_end)
        (connector_end)))
    (line_comment)
    (structure_def 'Person1'
      (feature_def var 'isLicensed' : 'Boolean' multiplicity))
    (structure_def 'Car1'
      (feature_def var 'driver' : 'Person1' multiplicity)
      (feature_def portion :>> 'startShot'
        (feature_def var :>> 'driver' multiplicity))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (feature_def portion 'operated' multiplicity :> 'timeSlices'
        (feature_def var :>> 'driver' multiplicity
          (feature_def var :>> 'isLicensed' value)))
      (feature_def abstract var 'carParts' multiplicity)
      (feature_def var 'engine' multiplicity :> 'carParts')
      (feature_def var 'transmission' multiplicity :> 'carParts')
      (malformed)
      (connector_def 'drive'
        (connector_end)
        (connector_end)))
    (line_comment)
    (line_comment)
    (structure_def 'Person1_'
      (line_comment)
      (feature_def member 'isLicensed' : 'Boolean' multiplicity featured by 'Person_snapshots'
        (feature_def member 'Person_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Person1_'))
      (feature_def member 'name' : 'String' multiplicity featured by 'Person_snapshots'
        (feature_def member 'Person_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Person1_')))
    (structure_def 'Car1_'
      (line_comment)
      (feature_def member 'driver' : 'Person1_' multiplicity featured by 'Car_snapshots'
        (feature_def member 'Car_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Car1_'))
      (feature_def portion :>> 'startShot'
        (line_comment)
        (feature_def member :>> 'Car1_::driver' multiplicity featured by 'Car_startShot_snapshots'
          (feature_def member 'Car_startShot_snapshots' :>> 'Car_snapshots' featured by 'Car1_::startShot')))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (feature_def portion 'operated' multiplicity :> 'timeSlices'
        (line_comment)
        (feature_def member :>> 'Car1_::driver' multiplicity featured by 'Car_operated_snapshots'
          (feature_def member 'Car_operated_snapshots' :>> 'Car_snapshots' featured by 'Car1_::operated')
          (line_comment)
          (feature_def member 'isLicensed1' :>> 'Person1_::isLicensed' value featured by 'Car_operated_driver_snapshots'
            (feature_def member 'Car_operated_driver_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Car1_::operated::driver'))))
      (line_comment)
      (feature_def member abstract 'carParts' multiplicity featured by 'Car_snapshots'
        (feature_def member 'Car_snapshots' :>> 'Occurrences::Occurrence::snapshots' featured by 'Car1_'))
      (line_comment)
      (feature_def member 'engine' multiplicity :> 'carParts' featured by 'Car_snapshots1'
        (feature_def member 'Car_snapshots1' :>> 'Occurrences::Occurrence::snapshots' featured by 'Car1_'))
      (line_comment)
      (feature_def member 'transmission' multiplicity :> 'carParts' featured by 'Car_snapshots1'
        (feature_def member 'Car_snapshots1' :>> 'Occurrences::Occurrence::snapshots' featured by 'Car1_'))
      (line_comment)
      (malformed))))
~~~
# FORMAT
~~~sysml
package TimeVaryingCarDriver {
    private import ScalarValues::*;
    
    // Example model without variable features.
    
    struct Person0 {
        feature isLicensed : Boolean [0..1];
    }
    
    struct Car0 {
        feature driver : Person0 [0..1];
        
        portion :>> startShot {
        	feature :>> driver [0];
        }
        
        succession first startShot then operated; 

        portion operated [0..*] :> timeSlices {
            feature :>> driver [1] {
                feature :>> isLicensed = true;
            }
        }

        abstract feature carParts [0..*];
        feature engine [1] :> carParts;
        feature transmission [1] :> carParts;
        
        connector drive from engine to transmission;      
    }
    
    // Example model with "variable features" identified
    
    struct Person1 {
        var feature isLicensed : Boolean [1];
    }
    
    struct Car1 {
        var feature driver : Person1 [0..1];
        
        portion :>> startShot {
        	var feature :>> driver [0]; 
        }
        
        succession first startShot then operated; 

        portion operated [0..*] :> timeSlices {
            var feature :>> driver [1] {
                var feature :>> isLicensed = true;
            }
        }
        
        abstract var feature carParts [0..*];
        var feature engine [1] :> carParts;
        var feature transmission [1] :> carParts;
        
        var connector drive from engine to transmission;
    }
    
  	// Semantic equivalent of implied relationships for variable features in
  	// the previous model
  
    struct Person1_ {
        // var feature isLicensed : Boolean [1];
        member feature isLicensed : Boolean [1] featured by Person_snapshots {
            member feature Person_snapshots :>> Occurrences::Occurrence::snapshots featured by Person1_;
        }
        member feature name : String [1] featured by Person_snapshots {
            member feature Person_snapshots :>> Occurrences::Occurrence::snapshots featured by Person1_;
        }
    }
    
    struct Car1_ {
        // var feature driver : Person [0..1];
        member feature driver : Person1_ [0..1] featured by Car_snapshots {
            member feature Car_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }
        
        portion :>> startShot {
        	// var feature :>> driver [0];
           	member feature :>> Car1_::driver [0] featured by Car_startShot_snapshots {
                member feature Car_startShot_snapshots :>> Car_snapshots featured by Car1_::startShot;
        	}
        }
        
        succession first startShot then operated; 

        portion operated [0..*] :> timeSlices {
            // var feature :>> driver [1]
            member feature :>> Car1_::driver [1] featured by Car_operated_snapshots {
                member feature Car_operated_snapshots :>> Car_snapshots featured by Car1_::operated;
                // var feature :>> isLicensed = true;
                member feature isLicensed1 :>> Person1_::isLicensed featured by Car_operated_driver_snapshots = true {
                    member feature Car_operated_driver_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_::operated::driver;
                }
            }
        }

        // var abstract feature carParts [0..*];
        member abstract feature carParts [0..*] featured by Car_snapshots {
            member feature Car_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }
        
        // var feature engine [1];
        member feature engine [1] :> carParts featured by Car_snapshots1 {
            member feature Car_snapshots1 :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }       
        
        // var feature transmission [1];
        member feature transmission [1] :> carParts featured by Car_snapshots1 {
            member feature Car_snapshots1 :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }       
        
        // var connector drive from engine to transmission;
        member connector drive featured by Car_snapshots from engine to transmission {
            member feature Car_snapshots :>> Occurrences::Occurrence::snapshots featured by Car1_;
        }
    }
    
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.unexpected_token
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Person_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Person_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'Car_startShot_snapshots'
semantic.unresolved_name 'Car1_::startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Car_operated_snapshots'
semantic.unresolved_name 'Car_operated_driver_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots1'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots1'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.unexpected_token
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Person_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Person_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'Car_startShot_snapshots'
semantic.unresolved_name 'Car1_::startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'Car_operated_snapshots'
semantic.unresolved_name 'Car_operated_driver_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots1'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
semantic.unresolved_name 'Car_snapshots1'
semantic.unresolved_name 'Occurrences::Occurrence::snapshots'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver"))) (name "TimeVaryingCarDriver") (declared-name "TimeVaryingCarDriver")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::*"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Car0"))) (name "Car0") (declared-name "Car0"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Car1"))) (name "Car1") (declared-name "Car1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Car1_"))) (name "Car1_") (declared-name "Car1_"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Person0"))) (name "Person0") (declared-name "Person0"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Person1"))) (name "Person1") (declared-name "Person1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Person1_"))) (name "Person1_") (declared-name "Person1_"))
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
