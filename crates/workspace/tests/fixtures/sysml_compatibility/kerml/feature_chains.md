# META
~~~ini
description=KerML Simple Tests: FeatureChains
type=file
~~~
# SOURCE
~~~kerml
package FeatureChains {
	classifier F {
		feature a : A;
	}
	  
	feature f : F;
	  
	classifier A {
		feature g = f.a;
	}
	  
	classifier B {
	  	feature f : F;
	  	feature a : A;
	}
	  
	feature b : B {
	  	connector f.a to a.g;
	  	binding f.a = a.g;
	}
	  
	feature g subsets f.a;
	subset g.g subsets b.f.a;
	redefinition b.f redefines b.a;
	  
	subtype g.g specializes b.f.a;
	
	disjoint b.f.a from b.a;
	
	feature h1 unions f, b.f, b.a;
	feature h2 differences b.f, b.a intersects f.a, g disjoint from h1;
	
	feature b_f_a chains b chains f.a;
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwClassifier,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwConnector,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBinding,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwRedefinition,Ident,Dot,Ident,KwRedefines,Ident,Dot,Ident,Semicolon,
KwSubtype,Ident,Dot,Ident,KwSpecializes,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwDisjoint,Ident,Dot,Ident,Dot,Ident,KwFrom,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwUnions,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwDifferences,Ident,Dot,Ident,Comma,Ident,Dot,Ident,KwIntersects,Ident,Dot,Ident,Comma,Ident,KwDisjoint,KwFrom,Ident,Semicolon,
KwFeature,Ident,KwChains,Ident,KwChains,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'FeatureChains'
    (classifier_def 'F'
      (feature_def 'a' : 'A'))
    (feature_def 'f' : 'F')
    (classifier_def 'A'
      (feature_def 'g' value))
    (classifier_def 'B'
      (feature_def 'f' : 'F')
      (feature_def 'a' : 'A'))
    (feature_def 'b' : 'B'
      (connector_def
        (connector_end)
        (connector_end))
      (binding_connector
        (connector_end)
        (connector_end)))
    (feature_def 'g' :> 'f.a')
    (subsetting_decl specific 'g.g' general 'b.f.a')
    (redefinition_decl specific 'b.f' general 'b.a')
    (specialization_decl specific 'g.g' general 'b.f.a')
    (disjoining_decl specific 'b.f.a' general 'b.a')
    (feature_def 'h1' unions 'f', 'b.f', 'b.a')
    (feature_def 'h2' differences 'b.f', 'b.a' intersects 'f.a', 'g' disjoint from 'h1')
    (feature_def 'b_f_a' chains 'b' chains 'f.a')))
~~~
# FORMAT
~~~sysml
package FeatureChains {
	classifier F {
		feature a : A;
	}
	  
	feature f : F;
	  
	classifier A {
		feature g = f.a;
	}
	  
	classifier B {
	  	feature f : F;
	  	feature a : A;
	}
	  
	feature b : B {
	  	connector f.a to a.g;
	  	binding f.a = a.g;
	}
	  
	feature g subsets f.a;
	subset g.g subsets b.f.a;
	redefinition b.f redefines b.a;
	  
	subtype g.g specializes b.f.a;
	
	disjoint b.f.a from b.a;
	
	feature h1 unions f, b.f, b.a;
	feature h2 differences b.f, b.a intersects f.a, g disjoint from h1;
	
	feature b_f_a chains b chains f.a;
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FeatureChains"))) (name "FeatureChains") (declared-name "FeatureChains")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "FeatureChains::A"))) (name "A") (declared-name "A"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "FeatureChains::B"))) (name "B") (declared-name "B"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "FeatureChains::F"))) (name "F") (declared-name "F"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureChains::b"))) (name "b") (declared-name "b"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureChains::f"))) (name "f") (declared-name "f"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureChains::g"))) (name "g") (declared-name "g"))
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
