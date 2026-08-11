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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "base.md"
    (diagnostics
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "03a5b99dd3590504feb8a42f066534c12d05c48a56c261b7cb6fa0809a6e63d7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Base"))) (kind "package") (name "Base") (declared-name "Base"))
    (element (id (node (document "d0") (qualified-name "Base::Anything"))) (kind "classifier decl") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::DataValue"))) (kind "kermlDecl") (name "DataValue") (declared-name "DataValue") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::dataValues"))) (kind "feature decl") (name "dataValues") (declared-name "dataValues") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::exactlyOne"))) (kind "kermlDecl") (name "exactlyOne") (declared-name "exactlyOne") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::naturals"))) (kind "feature decl") (name "naturals") (declared-name "naturals") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::oneToMany"))) (kind "kermlDecl") (name "oneToMany") (declared-name "oneToMany") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::things"))) (kind "feature decl") (name "things") (declared-name "things") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::zeroOrOne"))) (kind "kermlDecl") (name "zeroOrOne") (declared-name "zeroOrOne") (parent (node (document "d0") (qualified-name "Base"))))
    (element (id (node (document "d0") (qualified-name "Base::zeroToMany"))) (kind "kermlDecl") (name "zeroToMany") (declared-name "zeroToMany") (parent (node (document "d0") (qualified-name "Base"))))
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
