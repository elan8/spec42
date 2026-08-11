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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_chains.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 22 1) (end 22 265))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5d93bcb75ee63af3ca04d532d8582eb105b36f9ce0cfac97527cefb4d2cbc901") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureChains"))) (kind "package") (name "FeatureChains") (declared-name "FeatureChains") (range (start (line 0) (character 0)) (end (line 0) (character 550))))
    (element (id (node (document "d0") (qualified-name "FeatureChains::A"))) (kind "classifier decl") (name "A") (declared-name "A") (range (start (line 7) (character 1)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "FeatureChains"))))
    (element (id (node (document "d0") (qualified-name "FeatureChains::B"))) (kind "classifier decl") (name "B") (declared-name "B") (range (start (line 11) (character 1)) (end (line 11) (character 56))) (parent (node (document "d0") (qualified-name "FeatureChains"))))
    (element (id (node (document "d0") (qualified-name "FeatureChains::F"))) (kind "classifier decl") (name "F") (declared-name "F") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "FeatureChains"))))
    (element (id (node (document "d0") (qualified-name "FeatureChains::b"))) (kind "feature decl") (name "b") (declared-name "b") (range (start (line 16) (character 1)) (end (line 16) (character 68))) (parent (node (document "d0") (qualified-name "FeatureChains"))))
    (element (id (node (document "d0") (qualified-name "FeatureChains::f"))) (kind "feature decl") (name "f") (declared-name "f") (range (start (line 5) (character 1)) (end (line 5) (character 15))) (parent (node (document "d0") (qualified-name "FeatureChains"))))
    (element (id (node (document "d0") (qualified-name "FeatureChains::g"))) (kind "feature decl") (name "g") (declared-name "g") (range (start (line 21) (character 1)) (end (line 21) (character 23))) (parent (node (document "d0") (qualified-name "FeatureChains"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
