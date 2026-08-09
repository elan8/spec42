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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAbstract,KwType,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwType,KwAll,Ident,KwSpecializes,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwType,Ident,OpenSquare,DecimalValue,CloseSquare,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwType,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwPrivate,KwPackage,Ident,OpenCurly,
KwType,Ident,KwSpecializes,Ident,Semicolon,
CloseCurly,
KwProtected,KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwType,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwSpecialization,Ident,KwSubtype,Ident,KwSpecializes,Ident,Semicolon,
KwSpecialization,KwSubtype,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwType,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwIn,KwFeature,Ident,Semicolon,
CloseCurly,
KwType,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwType,Ident,KwSpecializes,Ident,ColonColon,Ident,Semicolon,
KwConjugation,Ident,KwConjugate,Ident,KwConjugates,Ident,Semicolon,
KwConjugation,Ident,KwConjugate,Ident,Tilde,Ident,Semicolon,
KwType,Ident,KwConjugates,Ident,Semicolon,
KwType,Ident,Tilde,Ident,Semicolon,
KwType,Ident,ColonGt,Ident,KwDisjoint,KwFrom,Ident,Semicolon,
KwType,Ident,ColonGt,Ident,ColonColon,Ident,KwUnions,Ident,Comma,Ident,Semicolon,
KwType,Ident,ColonGt,Ident,ColonColon,Ident,KwIntersects,Ident,Comma,Ident,Semicolon,
KwType,Ident,ColonGt,Ident,ColonColon,Ident,KwDifferences,Ident,Comma,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Types'
    (type_def abstract 'A' :> 'Base::Anything')
    (type_def all 'x' :> 'A', 'Base::things')
    (line_comment)
    (type_def 'Singleton' multiplicity     (multiplicity_range) :> 'Base::Anything')
    (type_def 'Super' :> 'Base::Anything'
      (package_def private 'P'
        (type_def 'Sub' :> 'Super'))
      (feature_def protected 'f' : 'P::Sub'))
    (type_def 'B' :> 'Base::Anything')
    (specialization_decl specialization 'Gen' specific 'A' general 'B')
    (malformed)
    (specialization_decl specific 'x' general 'Base::things')
    (type_def 'Original' :> 'Base::Anything'
      (feature_def in 'Input'))
    (type_def 'Conjugate1' :> 'Base::Anything')
    (type_def 'Conjugate2' :> 'Base::Anything')
    (conjugation_decl 'c1' specific 'Conjugate1' general 'Original')
    (conjugation_decl 'c2' specific 'Conjugate2' general 'Original')
    (type_def 'Conjugate3' ~ 'Original')
    (type_def 'Conjugate4' ~ 'Conjugate1')
    (type_def 'C' :> 'B' disjoint from 'A')
    (type_def 'D' :> 'Base::Anything' unions 'A', 'B')
    (type_def 'E' :> 'Base::Anything' intersects 'A', 'B')
    (type_def 'F' :> 'Base::Anything' differences 'A', 'B')))
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
# EXPECTED
~~~
parse.unexpected_token
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::things'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
~~~
# PROBLEMS
~~~
parse.unexpected_token
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::things'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Base::Anything'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Types"))) (name "Types") (declared-name "Types"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
