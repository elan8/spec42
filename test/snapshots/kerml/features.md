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
  (document "features.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 15 1) (end 15 178))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 41 1) (end 41 823))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a9b7810e5bf91e8098856ecdb54a0861afd9a020ab2bb994b7013db3ddb479d3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Features"))) (kind "package") (name "Features") (declared-name "Features"))
    (element (id (node (document "d0") (qualified-name "Features::A"))) (kind "classifier decl") (name "A") (declared-name "A") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::B"))) (kind "classifier decl") (name "B") (declared-name "B") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::C"))) (kind "classifier decl") (name "C") (declared-name "C") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::Fuel"))) (kind "classifier decl") (name "Fuel") (declared-name "Fuel") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::Tanks"))) (kind "classifier decl") (name "Tanks") (declared-name "Tanks") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::adult"))) (kind "feature decl") (name "adult") (declared-name "adult") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::child"))) (kind "feature decl") (name "child") (declared-name "child") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::f"))) (kind "feature decl") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::g"))) (kind "feature decl") (name "g") (declared-name "g") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::mother"))) (kind "feature decl") (name "mother") (declared-name "mother") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::parent12"))) (kind "feature decl") (name "parent12") (declared-name "parent12") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::person"))) (kind "feature decl") (name "person") (declared-name "person") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::x1"))) (kind "feature decl") (name "x1") (declared-name "x1") (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::y"))) (kind "feature decl") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "Features"))))
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
