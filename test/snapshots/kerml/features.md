# META
~~~ini
description=KerML Simple Tests: Features
type=file
~~~
# SOURCE
~~~kerml
package Features {
	classifier A;
	classifier B;
	
	feature f;
	feature g;
	
	feature x typed by A, B references f subsets g;
	
	// Equivalent declaration:
	feature x1 subsets g typed by A subsets f typed by B;
	
	classifier C;
	
	feature y;
	featuring F of y by C;
	
	feature y1 : A :> x featured by C;
	
	feature z unions f, g disjoint from y;
	feature z1 intersects f,g differences y, y1, z;
	
	classifier Person;
	
	abstract feature person : Person; // Default subsets Base::things.
	feature child subsets person;
	
	feature adult differences person, child;
	
	classifier Fuel;
	
	classifier Tanks {
	    feature fuelInPort {
	        in feature fuelFlow : Fuel;
	    }
	    feature fuelOutPort ~ fuelInPort;
	}
	
	feature parent[1..2] : Person;
	feature mother : Person[1] :> parent;
	
	specialization t1 typing f typed by B;
	specialization t2 typing g : A;
	
	specialization Sub subset parent subsets person;
	specialization subset mother subsets parent;
	
	classifier LegalRecord {
		feature guardian[1];
	}
	
    class RegisteredAsset {
        composite var feature identifier[0..1];
    }
    
    classifier Vehicle :> RegisteredAsset {
        derived var feature vin[1] = identifier;
        
        var feature v : Vehicle;
        binding vin = v.vin;
        var feature w = v.vin;
        
        feature x = vin;
        binding x = vin;
    }
	feature legalIdentification;
	
	specialization Redef redefinition LegalRecord::guardian redefines parent;
	specialization redefinition Vehicle::vin redefines RegisteredAsset::identifier;
	
	redefinition Vehicle::vin redefines legalIdentification; 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/features.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 1) (end 1 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 1) (end 2 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 4 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 1) (end 5 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 1) (end 7 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 10 1) (end 10 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 10 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 1) (end 12 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 14 11))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 15 1) (end 17 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 17 1) (end 17 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 1) (end 17 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 19 1) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 1) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 20 1) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 1) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 22 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 24 1) (end 24 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 24 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 1) (end 25 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 27 1) (end 27 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 1) (end 27 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 31 1) (end 36 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 1) (end 36 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 38 1) (end 38 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 38 1) (end 38 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 39 1) (end 39 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 1) (end 39 38))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 1) (end 51 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 4) (end 53 5))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 52 8) (end 53 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 55 4) (end 64 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 55 4) (end 64 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 65 1) (end 65 29))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 67 1) (end 71 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:00b61c0e836d1cea4e3d75c9e05025a79f812727ef20f86d6a231fe523e3fdcd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/features.md") (qualified-name "Features"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
