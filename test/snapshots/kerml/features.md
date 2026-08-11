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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,KwTyped,KwBy,Ident,Comma,Ident,KwReferences,Ident,KwSubsets,Ident,Semicolon,
LineComment,
KwFeature,Ident,KwSubsets,Ident,KwTyped,KwBy,Ident,KwSubsets,Ident,KwTyped,KwBy,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeaturing,Ident,KwOf,Ident,KwBy,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,ColonGt,Ident,KwFeatured,KwBy,Ident,Semicolon,
KwFeature,Ident,KwUnions,Ident,Comma,Ident,KwDisjoint,KwFrom,Ident,Semicolon,
KwFeature,Ident,KwIntersects,Ident,Comma,Ident,KwDifferences,Ident,Comma,Ident,Comma,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwAbstract,KwFeature,Ident,Colon,Ident,Semicolon,LineComment,
KwFeature,Ident,KwSubsets,Ident,Semicolon,
KwFeature,Ident,KwDifferences,Ident,Comma,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,OpenCurly,
KwIn,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Tilde,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
KwSpecialization,Ident,KwTyping,Ident,KwTyped,KwBy,Ident,Semicolon,
KwSpecialization,Ident,KwTyping,Ident,Colon,Ident,Semicolon,
KwSpecialization,Ident,KwSubset,Ident,KwSubsets,Ident,Semicolon,
KwSpecialization,KwSubset,Ident,KwSubsets,Ident,Semicolon,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwComposite,KwVar,KwFeature,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwClassifier,Ident,ColonGt,Ident,OpenCurly,
KwDerived,KwVar,KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,Semicolon,
KwBinding,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwVar,KwFeature,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,Eq,Ident,Semicolon,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Semicolon,
KwSpecialization,Ident,KwRedefinition,Ident,ColonColon,Ident,KwRedefines,Ident,Semicolon,
KwSpecialization,KwRedefinition,Ident,ColonColon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwRedefinition,Ident,ColonColon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Features'
    (classifier_def 'A')
    (classifier_def 'B')
    (feature_def 'f')
    (feature_def 'g')
    (feature_def 'x' : 'A', 'B' references 'f' :> 'g')
    (line_comment)
    (feature_def 'x1' :> 'g' : 'A' :> 'f' : 'B')
    (classifier_def 'C')
    (feature_def 'y')
    (type_featuring_decl)
    (feature_def 'y1' : 'A' :> 'x' featured by 'C')
    (feature_def 'z' unions 'f', 'g' disjoint from 'y')
    (feature_def 'z1' intersects 'f', 'g' differences 'y', 'y1', 'z')
    (classifier_def 'Person')
    (feature_def abstract 'person' : 'Person')
    (line_comment)
    (feature_def 'child' :> 'person')
    (feature_def 'adult' differences 'person', 'child')
    (classifier_def 'Fuel')
    (classifier_def 'Tanks'
      (feature_def 'fuelInPort'
        (feature_def in 'fuelFlow' : 'Fuel'))
      (feature_def 'fuelOutPort' ~ fuelInPort))
    (feature_def 'parent' multiplicity : 'Person')
    (feature_def 'mother' : 'Person' multiplicity :> 'parent')
    (feature_typing_decl specialization 't1' specific 'f' general 'B')
    (feature_typing_decl specialization 't2' specific 'g' general 'A')
    (subsetting_decl specialization 'Sub' specific 'parent' general 'person')
    (malformed)
    (subsetting_decl specific 'mother' general 'parent')
    (classifier_def 'LegalRecord'
      (feature_def 'guardian' multiplicity))
    (class_def 'RegisteredAsset'
      (feature_def composite var 'identifier' multiplicity))
    (classifier_def 'Vehicle' :> 'RegisteredAsset'
      (feature_def derived var 'vin' multiplicity value)
      (feature_def var 'v' : 'Vehicle')
      (binding_connector
        (connector_end)
        (connector_end))
      (feature_def var 'w' value)
      (feature_def 'x' value)
      (binding_connector
        (connector_end)
        (connector_end)))
    (feature_def 'legalIdentification')
    (redefinition_decl specialization 'Redef' specific 'LegalRecord::guardian' general 'parent')
    (malformed)
    (redefinition_decl specific 'Vehicle::vin' general 'RegisteredAsset::identifier')
    (redefinition_decl specific 'Vehicle::vin' general 'legalIdentification')))
~~~
# EXPECTED
~~~
parse.unexpected_token
parse.unexpected_token
semantic.ambiguous_member 'malformed'
~~~
# PROBLEMS
~~~
parse.unexpected_token
parse.unexpected_token
semantic.ambiguous_member 'malformed'
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
    (element (id (node (document "d0") (qualified-name "Features"))) (kind "package") (name "Features") (declared-name "Features") (range (start (line 0) (character 0)) (end (line 0) (character 1615))))
    (element (id (node (document "d0") (qualified-name "Features::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 1) (character 1)) (end (line 1) (character 14))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 2) (character 1)) (end (line 2) (character 14))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::C"))) (kind "classifier decl") (name "C") (declared-name "C") (range (start (line 12) (character 1)) (end (line 12) (character 14))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::Fuel"))) (kind "classifier decl") (name "Fuel") (declared-name "Fuel") (range (start (line 29) (character 1)) (end (line 29) (character 17))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::Tanks"))) (kind "classifier decl") (name "Tanks") (declared-name "Tanks") (range (start (line 31) (character 1)) (end (line 31) (character 131))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::adult"))) (kind "feature decl") (name "adult") (declared-name "adult") (range (start (line 27) (character 1)) (end (line 27) (character 41))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::child"))) (kind "feature decl") (name "child") (declared-name "child") (range (start (line 25) (character 1)) (end (line 25) (character 30))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::f"))) (kind "feature decl") (name "f") (declared-name "f") (range (start (line 4) (character 1)) (end (line 4) (character 11))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::g"))) (kind "feature decl") (name "g") (declared-name "g") (range (start (line 5) (character 1)) (end (line 5) (character 11))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::mother"))) (kind "feature decl") (name "mother") (declared-name "mother") (range (start (line 39) (character 1)) (end (line 39) (character 38))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::parent12"))) (kind "feature decl") (name "parent12") (declared-name "parent12") (range (start (line 38) (character 1)) (end (line 38) (character 31))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::person"))) (kind "feature decl") (name "person") (declared-name "person") (range (start (line 24) (character 1)) (end (line 24) (character 34))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 7) (character 1)) (end (line 7) (character 48))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::x1"))) (kind "feature decl") (name "x1") (declared-name "x1") (range (start (line 10) (character 1)) (end (line 10) (character 54))) (parent (node (document "d0") (qualified-name "Features"))))
    (element (id (node (document "d0") (qualified-name "Features::y"))) (kind "feature decl") (name "y") (declared-name "y") (range (start (line 14) (character 1)) (end (line 14) (character 11))) (parent (node (document "d0") (qualified-name "Features"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
