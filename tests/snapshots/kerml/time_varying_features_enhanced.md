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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 41))
      )
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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 20) (end 10 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 21) (end 17 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 24) (end 22 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 26) (end 32 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 28) (end 33 37))
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 28) (end 55 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 35) (end 70 45))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 96 8) (end 101 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 37) (end 101 47))
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 33) (end 108 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 28) (end 112 32))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:2618632173d101a5114915b275630b7abae623e5b2bc71f4f7d2e637b270fb36") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ExtendedOccurrences") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind) (value (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "x")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind) (value (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "x")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind) (value (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "y")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind) (value (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "x")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind bind) (value (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "y")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (kind kerml-structure) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedObject")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 0))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::inOperable"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::operated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "timeSlices")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "driver")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::speed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::transmission"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ExtendedOccurrence")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person::isLicensed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person::speed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ExtendedOccurrences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedObject")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::inOperable"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::operated"))) (kind subsetting) (ordinal 0))
      (authored-target "timeSlices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver")))))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::speed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (kind specialization) (ordinal 0))
      (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person::isLicensed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person::speed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::inOperable"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::operated"))) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 3)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t")))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1")))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (source inherited) (from (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")) (scopes any feature))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven")))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (source inherited) (from (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")) (scopes any feature))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))
      (type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (source direct))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (scopes any))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver")))
      (type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (source direct))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (scopes any))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::inOperable")))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::operated")))
      (featured-by (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car")))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (source inherited) (from (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver")) (scopes any feature))
      (supertype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 1 19) (end 1 41)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ExtendedOccurrences")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 47 19) (end 47 40)) (probe (position 47 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 48 19) (end 48 37)) (probe (position 48 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 3 17) (end 3 35)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 10 20) (end 10 29)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 11 28) (end 11 29)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 17 21) (end 17 31)) (probe (position 17 21))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 22 24) (end 22 33)) (probe (position 22 24))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 23 32) (end 23 33)) (probe (position 23 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 27 32) (end 27 33)) (probe (position 27 32))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 32 26) (end 32 36)) (probe (position 32 26))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::t1"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 33 28) (end 33 37)) (probe (position 33 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 34 36) (end 34 37)) (probe (position 34 36))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::x")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 38 36) (end 38 37)) (probe (position 38 36))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "CC1")) (named (kind kerml-feature) (name "t")) (named (kind kerml-feature) (name "t1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::CC1::t::y")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 50 17) (end 50 35)) (probe (position 50 17))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 118 19) (end 118 33)) (probe (position 118 19))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1"))) (kind specialization) (ordinal 0) (authored-target "ExtendedObject")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 126 17) (end 126 26)) (probe (position 126 17))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 127 25) (end 127 31)) (probe (position 127 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 135 23) (end 135 33)) (probe (position 135 23))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driven"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 136 25) (end 136 31)) (probe (position 136 25))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind kerml-structure) (name "Car1")) (named (kind kerml-feature) (name "driven")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 119 26) (end 119 32)) (probe (position 119 26))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car1::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 51 29) (end 51 35)) (probe (position 51 29))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 101 37) (end 101 47)) (probe (position 101 37))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::inOperable"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 70 35) (end 70 45)) (probe (position 70 35))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::operated"))) (kind subsetting) (ordinal 0) (authored-target "timeSlices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 71 28) (end 71 34)) (probe (position 71 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (path (named (kind package) (name "TimeVaryingFeaturesEnhanced")) (named (kind class-def) (name "Car")) (named (kind kerml-feature) (name "operated")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "driver")
      (outcome (status resolved) (target (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::driver")))))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 55 28) (end 55 32)) (probe (position 55 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Car::speed"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 107 20) (end 107 38)) (probe (position 107 20))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person"))) (kind specialization) (ordinal 0) (authored-target "ExtendedOccurrence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 108 33) (end 108 40)) (probe (position 108 33))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person::isLicensed"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/time_varying_features_enhanced.md") (range (start 112 28) (end 112 32)) (probe (position 112 28))
    (reference (id (source (node (document "memory://snapshot/time_varying_features_enhanced.md") (qualified-name "TimeVaryingFeaturesEnhanced::Person::speed"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
