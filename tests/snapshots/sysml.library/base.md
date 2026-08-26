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
  (document "memory://snapshot/base.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 50) (end 13 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 28) (end 56 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:93cb44b718e717a9ba8a8c5c0b69f4a8013cd7a555d270b1145d5fed5e3fc8dd") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the classifiers and features that provide the bases for the typing\n\t * of all elements in the language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t     * Anything is the top level generalized type in the language. \n\t     "))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t\t * The source of a SelfLink of this thing to itself. self is thus a feature that\n\t\t\t * relates everything to itself. It is also the value of the nested \"that\" feature\n\t\t\t * of all other things featured by this thing.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")) (featureChaining (reference "things::that")) (subsetting (reference "things")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (kind kerml-datatype) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * Value is the most general classifier of entities that are values that do not change\n\t\t * over time.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DataValue")) (redefinition (reference "Anything::self")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract nonunique) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * dataValues is a specialization of things restricted to type DataValue.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DataValue")) (subsetting (reference "things")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::exactlyOne"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t * exactlyOne is a multiplicity range requiring a cardinality of exactly one.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * naturals is a specialization of dataValues restricted to type Natural. \n\t\t * It is the root feature of all multiplicities, which map from a feature to\n\t\t * the set of Natural numbers representing allowable cardinalities of the feature.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Natural")) (subsetting (reference "dataValues")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::oneToMany"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))) (documentation (doc (text "\n\t\t * oneToMany is a multiplicity range allowing any cardinality of one or more.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers abstract nonunique) (multiplicity (lower 1) (upper unbounded))) (documentation (doc (text "\n\t\t * things is the top-level feature in the language.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t\t * For each value of things, the \"featuring instance\" of that value. \n\t\t\t * This is enforced by declaring Anything::self to be the chaining of things.that, \n\t\t\t * restricting it the single value of self.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Anything")))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::zeroOrOne"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t * zeroOrOne is a multiplicity range requiring a cardinality of zero or one.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::zeroToMany"))) (kind kerml-multiplicity) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * zeroToMany is a multiplicity range allowing any cardinality of zero or more\n\t\t * (that is, no restriction).\n\t\t "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind featureChaining) (ordinal 0))
      (authored-target "things::that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind subsetting) (ordinal 0))
      (authored-target "things")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (kind specialization) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind featureTyping) (ordinal 0))
      (authored-target "DataValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind redefinition) (ordinal 0))
      (authored-target "Anything::self")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind featureTyping) (ordinal 0))
      (authored-target "DataValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind subsetting) (ordinal 0))
      (authored-target "things")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (kind subsetting) (ordinal 0))
      (authored-target "dataValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that"))) (kind featureTyping) (ordinal 0))
      (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that"))) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self")) (scopes any))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::things")) (scopes any))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self")))
      (featured-by (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))
      (type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self")) (scopes any))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self")))
      (featured-by (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")))
      (type (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (source direct))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self")) (scopes any feature))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues")))
      (type (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (source direct))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals")))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::things")))
      (type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source direct))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self")) (scopes any feature))
      (subtype (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that")))
      (featured-by (node (document "memory://snapshot/base.md") (qualified-name "Base::things")))
      (type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (source direct))
      (supertype (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/base.md") (range (start 13 16) (end 13 24)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 13 50) (end 13 61)) (probe (position 13 50))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind featureChaining) (ordinal 0) (authored-target "things::that")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 13 36) (end 13 42)) (probe (position 13 36))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self"))) (kind subsetting) (ordinal 0) (authored-target "things")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 23 41) (end 23 49)) (probe (position 23 41))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue"))) (kind specialization) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 30 16) (end 30 25)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind featureTyping) (ordinal 0) (authored-target "DataValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 30 36) (end 30 50)) (probe (position 30 36))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue::self"))) (kind redefinition) (ordinal 0) (authored-target "Anything::self")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything::self")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 49 30) (end 49 39)) (probe (position 49 30))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind featureTyping) (ordinal 0) (authored-target "DataValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::DataValue")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 49 64) (end 49 70)) (probe (position 49 64))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues"))) (kind subsetting) (ordinal 0) (authored-target "things")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::things")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 56 28) (end 56 49)) (probe (position 56 28))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 56 64) (end 56 74)) (probe (position 56 64))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::naturals"))) (kind subsetting) (ordinal 0) (authored-target "dataValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::dataValues")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 33 26) (end 33 34)) (probe (position 33 26))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/base.md") (range (start 39 17) (end 39 25)) (probe (position 39 17))
    (reference (id (source (node (document "memory://snapshot/base.md") (qualified-name "Base::things::that"))) (kind featureTyping) (ordinal 0) (authored-target "Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/base.md") (qualified-name "Base::Anything")))))
    )
  )
)
~~~
