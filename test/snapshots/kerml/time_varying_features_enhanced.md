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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/time_varying_features_enhanced.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 17) (end 3 35))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 8) (end 10 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 10 8) (end 17 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 17 8) (end 45 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 47 19) (end 47 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 48 19) (end 48 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 50 17) (end 50 35))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 51 8) (end 55 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 55 8) (end 70 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 70 8) (end 86 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 86 8) (end 91 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 91 8) (end 96 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 96 8) (end 101 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 101 8) (end 104 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 104 8) (end 105 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 107 20) (end 107 38))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 108 8) (end 112 8))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 112 8) (end 116 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 118 19) (end 118 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 126 17) (end 126 26))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 133 5) (end 135 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 135 23) (end 135 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:2618632173d101a5114915b275630b7abae623e5b2bc71f4f7d2e637b270fb36") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ExtendedOccurrences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedObject"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 0))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ExtendedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 1 19) (end 1 41)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ExtendedOccurrences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 47 19) (end 47 40)) (probe (position 47 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 1)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 48 19) (end 48 37)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 2)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 3 17) (end 3 35)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 50 17) (end 50 35)) (probe (position 50 17))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 118 19) (end 118 33)) (probe (position 118 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (kind specialization) (ordinal 0) (authored-target "ExtendedObject")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 126 17) (end 126 26)) (probe (position 126 17))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 127 25) (end 127 31)) (probe (position 127 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 135 23) (end 135 33)) (probe (position 135 23))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 136 25) (end 136 31)) (probe (position 136 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 119 26) (end 119 32)) (probe (position 119 26))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))))
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 107 20) (end 107 38)) (probe (position 107 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
  )
)
~~~
