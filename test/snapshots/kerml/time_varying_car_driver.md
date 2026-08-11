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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "time_varying_car_driver.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 31))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "404e0d5861e8b40043847245c9a25842a0cf2972f48989cbe6779f95476cc8e5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver"))) (kind "package") (name "TimeVaryingCarDriver") (declared-name "TimeVaryingCarDriver") (range (start (line 0) (character 0)) (end (line 0) (character 4313))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 31))))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Car0"))) (kind "classifier decl") (name "Car0") (declared-name "Car0") (range (start (line 9) (character 4)) (end (line 9) (character 562))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Car1"))) (kind "classifier decl") (name "Car1") (declared-name "Car1") (range (start (line 37) (character 4)) (end (line 37) (character 597))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Car1_"))) (kind "classifier decl") (name "Car1_") (declared-name "Car1_") (range (start (line 72) (character 4)) (end (line 72) (character 2226))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Person0"))) (kind "classifier decl") (name "Person0") (declared-name "Person0") (range (start (line 5) (character 4)) (end (line 5) (character 71))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Person1"))) (kind "classifier decl") (name "Person1") (declared-name "Person1") (range (start (line 33) (character 4)) (end (line 33) (character 72))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))))
    (element (id (node (document "d0") (qualified-name "TimeVaryingCarDriver::Person1_"))) (kind "classifier decl") (name "Person1_") (declared-name "Person1_") (range (start (line 62) (character 4)) (end (line 62) (character 457))) (parent (node (document "d0") (qualified-name "TimeVaryingCarDriver"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TimeVaryingCarDriver::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 19)) (end (line 1) (character 31))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
