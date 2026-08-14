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
  (document "memory://snapshot/time_varying_car_driver.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 29) (end 6 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 20) (end 12 29))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 16 8) (end 18 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 35) (end 18 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 28) (end 20 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 33) (end 34 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 40 20) (end 40 29))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 44 8) (end 46 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 35) (end 46 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 32) (end 48 42))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 56 8) (end 57 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 64 8) (end 67 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 67 8) (end 70 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 74 8) (end 78 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 78 20) (end 78 29))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 80 12) (end 83 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 85 8) (end 87 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 35) (end 87 45))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 89 12) (end 96 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 99 8) (end 104 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 104 8) (end 109 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 109 8) (end 114 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 114 8) (end 117 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:962da675c07ef4eb952789bd3d1ec3ac95d2b89ad062d56222e98968990cb945") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 0))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "engine")) (connectorEnd (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person0"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::operated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isLicensed"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 0))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract var) (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person1"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::operated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isLicensed"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "carParts"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1_"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1_")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1_::operated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0::isLicensed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1::isLicensed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1_"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind connectorEnd) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind connectorEnd) (ordinal 1))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person0")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::operated"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "isLicensed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person1")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::operated"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "isLicensed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::transmission"))) (kind subsetting) (ordinal 0))
      (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1_")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1_::operated"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0::isLicensed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1::isLicensed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::engine"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::transmission"))) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::transmission"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 1 19) (end 1 34)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 12 20) (end 12 29)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 13 21) (end 13 27)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 28 29) (end 28 35)) (probe (position 28 29))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind connectorEnd) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 28 39) (end 28 51)) (probe (position 28 39))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::drive"))) (kind connectorEnd) (ordinal 1) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 10 25) (end 10 32)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person0")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 25 30) (end 25 38)) (probe (position 25 30))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::engine"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 18 35) (end 18 45)) (probe (position 18 35))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::operated"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 19 24) (end 19 30)) (probe (position 19 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::driver")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 20 28) (end 20 38)) (probe (position 20 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car0")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "isLicensed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 26 36) (end 26 44)) (probe (position 26 36))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::transmission"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car0::carParts")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 40 20) (end 40 29)) (probe (position 40 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 41 25) (end 41 31)) (probe (position 41 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 38 29) (end 38 36)) (probe (position 38 29))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person1")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 53 34) (end 53 42)) (probe (position 53 34))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::engine"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 46 35) (end 46 45)) (probe (position 46 35))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::operated"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 47 28) (end 47 34)) (probe (position 47 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::driver")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 48 32) (end 48 42)) (probe (position 48 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "isLicensed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 54 40) (end 54 48)) (probe (position 54 40))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::transmission"))) (kind subsetting) (ordinal 0) (authored-target "carParts")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1::carParts")))))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 78 20) (end 78 29)) (probe (position 78 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (path (named (kind package) (name "TimeVaryingCarDriver")) (named (kind kerml-structure) (name "Car1_")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 87 35) (end 87 45)) (probe (position 87 35))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Car1_::operated"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 6 29) (end 6 36)) (probe (position 6 29))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person0::isLicensed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_car_driver.md") (range (start 34 33) (end 34 40)) (probe (position 34 33))
    (reference (id (source (node (document "memory://snapshot/time_varying_car_driver.md") (qualified-name "TimeVaryingCarDriver::Person1::isLicensed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
)
~~~
