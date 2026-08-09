# META
~~~ini
description=KerML Enhancements: TimeVaryingFeaturesEnhanced
type=file
~~~
# SOURCE
~~~kerml
package TimeVaryingFeaturesEnhanced {
    private import ExtendedOccurrences::*;
    
    class CC1 :> ExtendedOccurrence {
        var feature x;
        //member feature x featured by CC1_snapshots {
        //    member feature CC1_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by CC1;
        //}
        
        // portions are not variable
        portion :>> startShot {
            var feature :>> x = 0;
            //member feature :>> CC1::x featured by CC1_startShot_snapshots = 0 {
            //    member feature CC1_startShot_snapshots :>> CC1_snapshots featured by CC1::startShot;
            //}
        }
        
        portion t :> timeSlices {
            var feature y;
            //member feature y featured by CC1_t_snapshots {
            //    member feature CC1_t_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by CC1::t;
            //}
            portion :>> startShot {
                var feature :>> x = 0;
                //member feature :>> CC1::x featured by CC1_t_startShot_snapshots = 0 {
                //    member feature CC1_t_startShot_snapshots :>> CC1_snapshots featured by CC1::t::startShot;
                //}
                var feature :>> y = 1;
                //member feature :>> CC1::t::y featured by CC1_t_startShot_snapshots = 1 {
                //    member feature CC1_t_startShot_snapshots :>> CC1_t_snapshots featured by CC1::t::startShot;
                //}
            }
            portion t1 :> timeSlices {
                portion :>> startShot {
                    var feature :>> x = 2;
                    //member feature :>> CC1::x featured by CC1_t_t1_startShot_snapshots = 2 {
                    //    member feature CC1_t_t1_startShot_snapshots :>> CC1_snapshots featured by CC1::t::t1::startShot;
                    //}
                    var feature :>> y = 3;
                    //member feature :>> CC1::t::y featured by CC1_t_t1_startShot_snapshots = 3 {
                    //    member feature CC1_t_t1_startShot_snapshots :>> CC1_t_snapshots featured by CC1::t::t1::startShot;
                    //}
                }
            }
        }
    }
    
    private import ScalarValues::Boolean;
    private import ScalarValues::Real;
    
    class Car :> ExtendedOccurrence {
        var feature driver : Person [0..1];
        //member feature driver : Person [0..1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}
        var feature speed : Real [1];
        //member feature speed : Real [1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}

        // bind the current speed to the current speed of the current driver
        // var binding driver.speed = speed;
        //member connector : Links::SelfLink featured by Car_snapshots {
        //	:>> that : Car_snapshots;
        //	end feature :>> thisThing references that.driver.while{interval = Car_snapshots::self}.speed;
        //	end feature :>> thisThing references that.driver.at{timeslices = Car_snapshots::self.moment}.speed;
        //	end feature :>> sameThing references that.speed;
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}
        
        portion operated [0..*] :> timeSlices {
            var feature :>> driver [1];
            //member feature :>> Car::driver [1] featured by Car_operated_snapshots {
            //    member feature Car_operated_snapshots :>> Car_snapshots featured by Car::operated;
                
                // var feature :>> isLicensed = true;
            //    member feature isLicensed1 :>> Person::isLicensed featured by Car_operated_driver_snapshots = true {
            //        member feature Car_operated_driver_snapshots :>> Person_snapshots featured by Car::operated::driver;
            //    }
            //}
            
            //portion :>> snapshots {
            //    public import operated;
            //}
        }
        
        var feature engine [1];
        //member feature engine [1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}       
        
        var feature transmission [1];
        //member feature transmission [1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}       
        
        var connector drive from engine to transmission;
        //member connector drive featured by Car_snapshots from engine to transmission {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots :> engine::Car_snapshots, transmission::Car_snapshots featured by Car;
        //}
        
        portion inOperable [0..1] :> timeSlices;
        
        // successions are not variable
        succession first operated then inOperable;
    }
    
    class Person :> ExtendedOccurrence {
        var feature isLicensed : Boolean[0..1];
        //member feature isLicensed : Boolean[0..1] featured by Person_snapshots {
        //    member feature Person_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Person;
        //}
        var feature speed : Real[1];
        //member feature speed : Real[1] featured by Person_snapshots {
        //    member feature Person_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Person;
        //}
    }
    
    struct Car1 :> ExtendedObject {  // May or may not be a life
	    var feature driver : Person [0..1];
	    //member feature driver : Person [0..1] featured by Car_snapshots {
	    //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car1;
	    //}
	  
	    // :>> timeSlices : Car;  <-- Don't do this!
	
	    portion :>> startShot {  // Not a kind of Car!
	        var feature :>> driver [0]; 
	        //member feature :>> driver : Person [0] featured by Car_startShot_snapshots {
	        //    member feature Car_startShot_snapshots :>> Car_snapshots featured by Car1::startShot;
	        //}
	    }
	
	    succession first startShot then driven; 
	
	    portion driven :> timeSlices {     
	        var feature :>> driver [1];
	        // No conflict with multiplicity! (driven just can't be startshot)
	        //member feature :>> driver : Person [1] featured by Car_driven_snapshots {
	        //    member feature Car_driven_snapshots :>> Car_snapshots featured by Car1::driven;
	        //}
    	}
	}
    
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,Ident,Semicolon,
LineComment,
LineComment,
LineComment,
LineComment,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
LineComment,
LineComment,
CloseCurly,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,Ident,Semicolon,
LineComment,
LineComment,
LineComment,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
LineComment,
LineComment,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
LineComment,
LineComment,
CloseCurly,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
KwPortion,ColonGtGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
LineComment,
LineComment,
KwVar,KwFeature,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
LineComment,
LineComment,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwClass,Ident,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
KwPortion,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
CloseCurly,
KwVar,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
KwVar,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
KwVar,KwConnector,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
LineComment,
LineComment,
LineComment,
KwPortion,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
LineComment,
KwSuccession,KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
KwClass,Ident,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
CloseCurly,
KwStruct,Ident,ColonGt,Ident,OpenCurly,LineComment,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
LineComment,
KwPortion,ColonGtGt,Ident,OpenCurly,LineComment,
KwVar,KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
CloseCurly,
KwSuccession,KwFirst,Ident,KwThen,Ident,Semicolon,
KwPortion,Ident,ColonGt,Ident,OpenCurly,
KwVar,KwFeature,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
LineComment,
LineComment,
LineComment,
LineComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TimeVaryingFeaturesEnhanced'
    (import_decl private 'ExtendedOccurrences::*')
    (class_def 'CC1' :> 'ExtendedOccurrence'
      (feature_def var 'x')
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def portion :>> 'startShot'
        (feature_def var :>> 'x' value)
        (line_comment)
        (line_comment)
        (line_comment))
      (feature_def portion 't' :> 'timeSlices'
        (feature_def var 'y')
        (line_comment)
        (line_comment)
        (line_comment)
        (feature_def portion :>> 'startShot'
          (feature_def var :>> 'x' value)
          (line_comment)
          (line_comment)
          (line_comment)
          (feature_def var :>> 'y' value)
          (line_comment)
          (line_comment)
          (line_comment))
        (feature_def portion 't1' :> 'timeSlices'
          (feature_def portion :>> 'startShot'
            (feature_def var :>> 'x' value)
            (line_comment)
            (line_comment)
            (line_comment)
            (feature_def var :>> 'y' value)
            (line_comment)
            (line_comment)
            (line_comment)))))
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::Real')
    (class_def 'Car' :> 'ExtendedOccurrence'
      (feature_def var 'driver' : 'Person' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def var 'speed' : 'Real' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def portion 'operated' multiplicity :> 'timeSlices'
        (feature_def var :>> 'driver' multiplicity)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment))
      (feature_def var 'engine' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def var 'transmission' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment)
      (malformed)
      (connector_def 'drive'
        (connector_end)
        (connector_end))
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def portion 'inOperable' multiplicity :> 'timeSlices')
      (line_comment)
      (succession_as_usage
        (connector_end)
        (connector_end)))
    (class_def 'Person' :> 'ExtendedOccurrence'
      (feature_def var 'isLicensed' : 'Boolean' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def var 'speed' : 'Real' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment))
    (structure_def 'Car1' :> 'ExtendedObject'
      (line_comment)
      (feature_def var 'driver' : 'Person' multiplicity)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (feature_def portion :>> 'startShot'
        (line_comment)
        (feature_def var :>> 'driver' multiplicity)
        (line_comment)
        (line_comment)
        (line_comment))
      (succession_as_usage
        (connector_end)
        (connector_end))
      (feature_def portion 'driven' :> 'timeSlices'
        (feature_def var :>> 'driver' multiplicity)
        (line_comment)
        (line_comment)
        (line_comment)
        (line_comment)))))
~~~
# FORMAT
~~~sysml
package TimeVaryingFeaturesEnhanced {
    private import ExtendedOccurrences::*;

    class CC1 :> ExtendedOccurrence {
        var feature x;
        //member feature x featured by CC1_snapshots {
        //    member feature CC1_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by CC1;
        //}

        // portions are not variable
        portion:>> startShot {
            var feature :>> x = 0;
            //member feature :>> CC1::x featured by CC1_startShot_snapshots = 0 {
            //    member feature CC1_startShot_snapshots :>> CC1_snapshots featured by CC1::startShot;
            //}
        }

        portion t:> timeSlices {
            var feature y;
            //member feature y featured by CC1_t_snapshots {
            //    member feature CC1_t_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by CC1::t;
            //}
            portion:>> startShot {
                var feature :>> x = 0;
                //member feature :>> CC1::x featured by CC1_t_startShot_snapshots = 0 {
                //    member feature CC1_t_startShot_snapshots :>> CC1_snapshots featured by CC1::t::startShot;
                //}
                var feature :>> y = 1;
                //member feature :>> CC1::t::y featured by CC1_t_startShot_snapshots = 1 {
                //    member feature CC1_t_startShot_snapshots :>> CC1_t_snapshots featured by CC1::t::startShot;
                //}
            }
            portion t1:> timeSlices {
                portion:>> startShot {
                    var feature :>> x = 2;
                    //member feature :>> CC1::x featured by CC1_t_t1_startShot_snapshots = 2 {
                    //    member feature CC1_t_t1_startShot_snapshots :>> CC1_snapshots featured by CC1::t::t1::startShot;
                    //}
                    var feature :>> y = 3;
                    //member feature :>> CC1::t::y featured by CC1_t_t1_startShot_snapshots = 3 {
                    //    member feature CC1_t_t1_startShot_snapshots :>> CC1_t_snapshots featured by CC1::t::t1::startShot;
                    //}
                }
            }
        }
    }

    private import ScalarValues::Boolean;
    private import ScalarValues::Real;

    class Car :> ExtendedOccurrence {
        var feature driver : Person [0..1];
        //member feature driver : Person [0..1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}
        var feature speed : Real [1];
        //member feature speed : Real [1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}

        // bind the current speed to the current speed of the current driver
        // var binding driver.speed = speed;
        //member connector : Links::SelfLink featured by Car_snapshots {
        //	:>> that : Car_snapshots;
        //	end feature :>> thisThing references that.driver.while{interval = Car_snapshots::self}.speed;
        //	end feature :>> thisThing references that.driver.at{timeslices = Car_snapshots::self.moment}.speed;
        //	end feature :>> sameThing references that.speed;
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}

        portion operated[0..*]:> timeSlices {
            var feature :>> driver [1];
            //member feature :>> Car::driver [1] featured by Car_operated_snapshots {
            //    member feature Car_operated_snapshots :>> Car_snapshots featured by Car::operated;

            // var feature :>> isLicensed = true;
            //    member feature isLicensed1 :>> Person::isLicensed featured by Car_operated_driver_snapshots = true {
            //        member feature Car_operated_driver_snapshots :>> Person_snapshots featured by Car::operated::driver;
            //    }
            //}

            //portion :>> snapshots {
            //    public import operated;
            //}
        }

        var feature engine[1];
        //member feature engine [1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}       

        var feature transmission[1];
        //member feature transmission [1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car;
        //}       

        var
        connector drive from engine to transmission;
        //member connector drive featured by Car_snapshots from engine to transmission {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots :> engine::Car_snapshots, transmission::Car_snapshots featured by Car;
        //}

        portion inOperable[0..1]:> timeSlices;

        // successions are not variable
        first operated then inOperable;
    }

    class Person :> ExtendedOccurrence {
        var feature isLicensed : Boolean [0..1];
        //member feature isLicensed : Boolean[0..1] featured by Person_snapshots {
        //    member feature Person_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Person;
        //}
        var feature speed : Real [1];
        //member feature speed : Real[1] featured by Person_snapshots {
        //    member feature Person_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Person;
        //}
    }

    struct Car1 :> ExtendedObject {
        // May or may not be a life
        var feature driver : Person [0..1];
        //member feature driver : Person [0..1] featured by Car_snapshots {
        //    member feature Car_snapshots :>> ExtendedOccurrences::ExtendedOccurrence::snapshots featured by Car1;
        //}

        // :>> timeSlices : Car;  <-- Don't do this!

        portion:>> startShot {
            // Not a kind of Car!
            var feature :>> driver [0];
            //member feature :>> driver : Person [0] featured by Car_startShot_snapshots {
            //    member feature Car_startShot_snapshots :>> Car_snapshots featured by Car1::startShot;
            //}
        }

        first startShot then driven;

        portion driven:> timeSlices {
            var feature :>> driver [1];
            // No conflict with multiplicity! (driven just can't be startshot)
            //member feature :>> driver : Person [1] featured by Car_driven_snapshots {
            //    member feature Car_driven_snapshots :>> Car_snapshots featured by Car1::driven;
            //}
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'ExtendedOccurrence'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'ExtendedOccurrence'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'ExtendedOccurrence'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ExtendedObject'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'ExtendedOccurrence'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'ExtendedOccurrence'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'ExtendedOccurrence'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ExtendedObject'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'timeSlices'
~~~
# SMG
~~~
(model
  (namespace
    (package 'TimeVaryingFeaturesEnhanced'
      (namespace_import private -> 'ExtendedOccurrences'[unresolved])
      (class_def 'CC1' :> 'ExtendedOccurrence'[unresolved]
        (feature_def 'x')
        (feature_def :>> 'startShot'[unresolved]
          (feature_def :>> 'TimeVaryingFeaturesEnhanced::CC1::x'[feature_def]
            (feature_value (=))))
        (feature_def 't' :> 'timeSlices'[unresolved]
          (feature_def 'y')
          (feature_def :>> 'startShot'[unresolved]
            (feature_def :>> 'TimeVaryingFeaturesEnhanced::CC1::x'[feature_def]
              (feature_value (=)))
            (feature_def :>> 'TimeVaryingFeaturesEnhanced::CC1::t::y'[feature_def]
              (feature_value (=))))
          (feature_def 't1' :> 'timeSlices'[unresolved]
            (feature_def :>> 'startShot'[unresolved]
              (feature_def :>> 'TimeVaryingFeaturesEnhanced::CC1::x'[feature_def]
                (feature_value (=)))
              (feature_def :>> 'TimeVaryingFeaturesEnhanced::CC1::t::y'[feature_def]
                (feature_value (=)))))))
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (class_def 'Car' :> 'ExtendedOccurrence'[unresolved]
        (feature_def 'driver' : 'TimeVaryingFeaturesEnhanced::Person'[class_def]
          (multiplicity_range [0..1]))
        (feature_def 'speed' : 'Real'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'operated' :> 'timeSlices'[unresolved]
          (multiplicity_range [0..*])
          (feature_def :>> 'TimeVaryingFeaturesEnhanced::Car::driver'[feature_def]
            (multiplicity_range [1])))
        (feature_def 'engine'
          (multiplicity_range [1]))
        (feature_def 'transmission'
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (connector_def 'drive'
          (connector_end 'engine')
          (connector_end 'transmission'))
        (feature_def 'inOperable' :> 'timeSlices'[unresolved]
          (multiplicity_range [0..1]))
        (succession_def
          (connector_end 'operated')
          (connector_end 'inOperable')))
      (class_def 'Person' :> 'ExtendedOccurrence'[unresolved]
        (feature_def 'isLicensed' : 'Boolean'[unresolved]
          (multiplicity_range [0..1]))
        (feature_def 'speed' : 'Real'[unresolved]
          (multiplicity_range [1])))
      (structure_def 'Car1' :> 'ExtendedObject'[unresolved]
        (feature_def 'driver' : 'TimeVaryingFeaturesEnhanced::Person'[class_def]
          (multiplicity_range [0..1]))
        (feature_def :>> 'startShot'[unresolved]
          (feature_def :>> 'TimeVaryingFeaturesEnhanced::Car1::driver'[feature_def]
            (multiplicity_range [0])))
        (succession_def
          (connector_end 'startShot')
          (connector_end 'driven'))
        (feature_def 'driven' :> 'timeSlices'[unresolved]
          (feature_def :>> 'TimeVaryingFeaturesEnhanced::Car1::driver'[feature_def]
            (multiplicity_range [1])))))))
~~~
