# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Base
type=file
~~~
# SOURCE
~~~kerml
standard library package Base {
	doc 
	/*
	 * This package defines the classifiers and features that provide the bases for the typing
	 * of all elements in the language.
	 */
	 		
	abstract classifier Anything {
		doc
		/*
	     * Anything is the top level generalized type in the language. 
	     */
		
		feature self: Anything[1] subsets things chains things.that {
			doc
			/*
			 * The source of a SelfLink of this thing to itself. self is thus a feature that
			 * relates everything to itself. It is also the value of the nested "that" feature
			 * of all other things featured by this thing.
			 */
		}
	}
	
	abstract datatype DataValue specializes Anything {
		doc
		/*
		 * Value is the most general classifier of entities that are values that do not change
		 * over time.
		 */
		
		feature self: DataValue redefines Anything::self;
	}
	
	abstract feature things: Anything [1..*] nonunique {
		doc
		/*
		 * things is the top-level feature in the language.
		 */

		feature that : Anything[1] {
			doc
			/*
			 * For each value of things, the "featuring instance" of that value. 
			 * This is enforced by declaring Anything::self to be the chaining of things.that, 
			 * restricting it the single value of self.
			 */			
		}
	}
	
	abstract feature dataValues: DataValue[0..*] nonunique subsets things {
		doc
		/*
		 * dataValues is a specialization of things restricted to type DataValue.
		 */
	}
		 
	abstract feature naturals: ScalarValues::Natural[0..*] subsets dataValues {
		doc
		/*
		 * naturals is a specialization of dataValues restricted to type Natural. 
		 * It is the root feature of all multiplicities, which map from a feature to
		 * the set of Natural numbers representing allowable cardinalities of the feature.
		 */
	}
	
	multiplicity exactlyOne [1..1] {
		doc
		/*
		 * exactlyOne is a multiplicity range requiring a cardinality of exactly one.
		 */
	}
	
	multiplicity zeroOrOne [0..1] {
		doc
		/*
		 * zeroOrOne is a multiplicity range requiring a cardinality of zero or one.
		 */		
	}
	
	multiplicity oneToMany [1..*] {
		doc
		/*
		 * oneToMany is a multiplicity range allowing any cardinality of one or more.
		 */
	}
	
	multiplicity zeroToMany [0..*] {
		doc
		/*
		 * zeroToMany is a multiplicity range allowing any cardinality of zero or more
		 * (that is, no restriction).
		 */
	}
	
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Natural'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Natural'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAbstract,KwClassifier,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,KwChains,Ident,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwDatatype,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwMultiplicity,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwMultiplicity,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwMultiplicity,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwMultiplicity,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Base'
    (documentation)
    (classifier_def abstract 'Anything'
      (documentation)
      (feature_def 'self' : 'Anything' multiplicity :> 'things' chains 'things.that'
        (documentation)))
    (datatype_def abstract 'DataValue' :> 'Anything'
      (documentation)
      (feature_def 'self' : 'DataValue' :>> 'Anything::self'))
    (feature_def abstract 'things' : 'Anything' multiplicity nonunique
      (documentation)
      (feature_def 'that' : 'Anything' multiplicity
        (documentation)))
    (feature_def abstract 'dataValues' : 'DataValue' multiplicity :> 'things' nonunique
      (documentation))
    (feature_def abstract 'naturals' : 'ScalarValues::Natural' multiplicity :> 'dataValues'
      (documentation))
    (multiplicity_def 'exactlyOne' multiplicity     (multiplicity_range)
      (documentation))
    (multiplicity_def 'zeroOrOne' multiplicity     (multiplicity_range)
      (documentation))
    (multiplicity_def 'oneToMany' multiplicity     (multiplicity_range)
      (documentation))
    (multiplicity_def 'zeroToMany' multiplicity     (multiplicity_range)
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package Base {
	doc 
	/*
	 * This package defines the classifiers and features that provide the bases for the typing
	 * of all elements in the language.
	 */
	 		
	abstract classifier Anything {
		doc
		/*
	     * Anything is the top level generalized type in the language. 
	     */
		
		feature self: Anything[1] subsets things chains things.that {
			doc
			/*
			 * The source of a SelfLink of this thing to itself. self is thus a feature that
			 * relates everything to itself. It is also the value of the nested "that" feature
			 * of all other things featured by this thing.
			 */
		}
	}
	
	abstract datatype DataValue specializes Anything {
		doc
		/*
		 * Value is the most general classifier of entities that are values that do not change
		 * over time.
		 */
		
		feature self: DataValue redefines Anything::self;
	}
	
	abstract feature things: Anything [1..*] nonunique {
		doc
		/*
		 * things is the top-level feature in the language.
		 */

		feature that : Anything[1] {
			doc
			/*
			 * For each value of things, the "featuring instance" of that value. 
			 * This is enforced by declaring Anything::self to be the chaining of things.that, 
			 * restricting it the single value of self.
			 */			
		}
	}
	
	abstract feature dataValues: DataValue[0..*] nonunique subsets things {
		doc
		/*
		 * dataValues is a specialization of things restricted to type DataValue.
		 */
	}
		 
	abstract feature naturals: ScalarValues::Natural[0..*] subsets dataValues {
		doc
		/*
		 * naturals is a specialization of dataValues restricted to type Natural. 
		 * It is the root feature of all multiplicities, which map from a feature to
		 * the set of Natural numbers representing allowable cardinalities of the feature.
		 */
	}
	
	multiplicity exactlyOne [1..1] {
		doc
		/*
		 * exactlyOne is a multiplicity range requiring a cardinality of exactly one.
		 */
	}
	
	multiplicity zeroOrOne [0..1] {
		doc
		/*
		 * zeroOrOne is a multiplicity range requiring a cardinality of zero or one.
		 */		
	}
	
	multiplicity oneToMany [1..*] {
		doc
		/*
		 * oneToMany is a multiplicity range allowing any cardinality of one or more.
		 */
	}
	
	multiplicity zeroToMany [0..*] {
		doc
		/*
		 * zeroToMany is a multiplicity range allowing any cardinality of zero or more
		 * (that is, no restriction).
		 */
	}
	
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Base"))) (name "Base") (declared-name "Base")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Base::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Base::DataValue"))) (name "DataValue") (declared-name "DataValue"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Base::_documentation"))) (name ""))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Base::dataValues"))) (name "dataValues") (declared-name "dataValues"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Base::exactlyOne"))) (name "exactlyOne") (declared-name "exactlyOne"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Base::naturals"))) (name "naturals") (declared-name "naturals"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Base::oneToMany"))) (name "oneToMany") (declared-name "oneToMany"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Base::things"))) (name "things") (declared-name "things"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Base::zeroOrOne"))) (name "zeroOrOne") (declared-name "zeroOrOne"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Base::zeroToMany"))) (name "zeroToMany") (declared-name "zeroToMany"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Base::_documentation"))) (to (node (document "d0") (qualified-name "Base"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
