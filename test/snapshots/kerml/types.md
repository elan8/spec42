# META
~~~ini
description=KerML Simple Tests: Types
type=file
~~~
# SOURCE
~~~kerml
package Types {
	abstract type A specializes Base::Anything;
	type all x specializes A, Base::things;
	
	// This Type has exactly one instance.
	type Singleton[1] specializes Base::Anything;
	
	type Super specializes Base::Anything {
	    private package P {
	        type Sub specializes Super;
	    }
	    protected feature f : P::Sub;
	}
	
	type B :> Base::Anything;
	
	specialization Gen subtype A specializes B;
	specialization subtype x :> Base::things;
	
	type Original specializes Base::Anything {
	    in feature Input; 
	}
	type Conjugate1 specializes Base::Anything;
	type Conjugate2 specializes Base::Anything;
	conjugation c1 conjugate Conjugate1 conjugates Original; 
	conjugation c2 conjugate Conjugate2 ~ Original; 
	
	type Conjugate3 conjugates Original;
	type Conjugate4 ~ Conjugate1;
	
	type C :> B disjoint from A;
	
	type D :> Base::Anything unions A, B;
	type E :> Base::Anything intersects A, B;
	type F :> Base::Anything differences A, B;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "types.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 1 1) (end 1 947))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package Types {
    abstract type A specializes Base::Anything;
    type all x specializes A, Base::things;

    // This Type has exactly one instance.
    type Singleton[1] specializes Base::Anything;

    type Super specializes Base::Anything {
        private package P {
            type Sub specializes Super;
        }
        protected feature f : P::Sub;
    }

    type B :> Base::Anything;

    specialization Gen subtype A specializes B;
    specialization subtype x :> Base::things;

    type Original specializes Base::Anything {
        in feature Input;
    }
    type Conjugate1 specializes Base::Anything;
    type Conjugate2 specializes Base::Anything;
    conjugation c1 conjugate Conjugate1 conjugates Original;
    conjugation c2 conjugate Conjugate2 ~ Original;

    type Conjugate3 conjugates Original;
    type Conjugate4 ~ Conjugate1;

    type C :> B disjoint from A;

    type D :> Base::Anything unions A, B;
    type E :> Base::Anything intersects A, B;
    type F :> Base::Anything differences A, B;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "386a281b8085663a83954ccebae369c6b2f8ed1b6cfae0794731a6ef307d4c3b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Types"))) (kind "package") (name "Types") (declared-name "Types") (range (start (line 0) (character 0)) (end (line 0) (character 964))))
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
