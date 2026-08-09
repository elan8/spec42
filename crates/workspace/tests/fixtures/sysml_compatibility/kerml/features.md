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
    feature z1 intersects f, g differences y, y1, z;

    classifier Person;

    abstract feature person : Person;
    // Default subsets Base::things.
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
    feature mother : Person [1] :> parent;

    specialization t1 typing f typed by B;
    specialization t2 typing g : A;

    specialization Sub subset parent subsets person;
    specialization
    subset mother subsets parent;

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
    specialization
    redefinition Vehicle::vin redefines RegisteredAsset::identifier;

    redefinition Vehicle::vin redefines legalIdentification;
}
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
# SMG
~~~
(model
  (namespace
    (package 'Features'
      (classifier_def 'A')
      (classifier_def 'B')
      (feature_def 'f')
      (feature_def 'g')
      (feature_def 'x' : 'Features::A'[classifier_def] : 'Features::B'[classifier_def] :> 'Features::f'[feature_def] :> 'Features::g'[feature_def])
      (feature_def 'x1' :> 'Features::g'[feature_def] : 'Features::A'[classifier_def] :> 'Features::f'[feature_def] : 'Features::B'[classifier_def])
      (classifier_def 'C')
      (feature_def 'y')
      (type_featuring_decl 'F')
      (feature_def 'y1' : 'Features::A'[classifier_def] :> 'Features::x'[feature_def])
      (feature_def 'z')
      (feature_def 'z1')
      (classifier_def 'Person')
      (feature_def abstract 'person' : 'Features::Person'[classifier_def])
      (feature_def 'child' :> 'Features::person'[feature_def])
      (feature_def 'adult')
      (classifier_def 'Fuel')
      (classifier_def 'Tanks'
        (feature_def 'fuelInPort'
          (feature_def in 'fuelFlow' : 'Features::Fuel'[classifier_def]))
        (feature_def 'fuelOutPort' ~ 'Features::Tanks::fuelInPort'[feature_def]))
      (feature_def 'parent' : 'Features::Person'[classifier_def]
        (multiplicity_range [1..2]))
      (feature_def 'mother' : 'Features::Person'[classifier_def] :> 'Features::parent'[feature_def]
        (multiplicity_range [1]))
      (feature_typing_decl 't1')
      (feature_typing_decl 't2')
      (subsetting_decl 'Sub')
      (not_implemented 'malformed')
      (subsetting_decl)
      (classifier_def 'LegalRecord'
        (feature_def 'guardian'
          (multiplicity_range [1])))
      (class_def 'RegisteredAsset'
        (feature_def composite 'identifier'
          (multiplicity_range [0..1])))
      (classifier_def 'Vehicle' :> 'Features::RegisteredAsset'[class_def]
        (feature_def derived 'vin'
          (multiplicity_range [1])
          (feature_value (=)))
        (feature_def 'v' : 'Features::Vehicle'[classifier_def])
        (binding_connector_def
          (connector_end 'vin')
          (connector_end 'v.vin'))
        (feature_def 'w'
          (feature_value (=)))
        (feature_def 'x'
          (feature_value (=)))
        (binding_connector_def
          (connector_end 'x')
          (connector_end 'vin')))
      (feature_def 'legalIdentification')
      (redefinition_decl 'Redef')
      (not_implemented 'malformed')
      (redefinition_decl)
      (redefinition_decl))))
~~~
