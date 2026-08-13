# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_11-Variable Length Collection Types
type=file
~~~
# SOURCE
~~~sysml
package '15_11-Variable Length Collection Types' {
	private import ScalarValues::*;
	private import Collections::*;
	
	part def SparePart;
	part def Person;
	
	/* Examples of declaring syntactic sugar-like names for instantiating collection types. */
	
	attribute def 'Bag<SparePart>' :> Bag {
		ref part :>> elements: SparePart;
	}
	
	attribute def 'List<Integer>' :> List {
		value :>> elements: Integer;
	}
	
	attribute def 'Set<String>' :> Set {
		attribute :>> elements: String;
	}
	
	attribute def 'OrderedSet<Person>' :> OrderedSet {
		ref part :>> elements: Person;
	}
	
	attribute def 'List<Set<Person>>' :> List {
		attribute :>> elements: Set {
			ref part :>> elements: Person;
		}
	}
	
	attribute def 'Array<Real>[4]' :> Array {
		attribute :>> elements: Real;
		attribute :>> dimensions = 4;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_11_variable_length_collection_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 35) (end 9 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 10 2) (end 10 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 13 34) (end 13 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 14 12) (end 14 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 22) (end 14 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 32) (end 17 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 18 16) (end 18 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 26) (end 18 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 21 39) (end 21 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 22 2) (end 22 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 25 38) (end 25 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 26 16) (end 26 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 26) (end 26 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 27 3) (end 27 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 35) (end 31 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 32 16) (end 32 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 26) (end 32 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 33 16) (end 33 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:58f108d64319267fd82221755588233f3add75f701fae8866de8c061dba5066d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Collections") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Array"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimensions"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Bag"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "List"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "List"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Set")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "OrderedSet"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Person"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Set"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::SparePart"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Collections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind specialization) (ordinal 0))
      (authored-target "Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind specialization) (ordinal 0))
      (authored-target "Bag")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind specialization) (ordinal 0))
      (authored-target "List")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind specialization) (ordinal 0))
      (authored-target "List")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Set")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind specialization) (ordinal 0))
      (authored-target "OrderedSet")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind specialization) (ordinal 0))
      (authored-target "Set")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unsupported)))
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
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Collections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 31 35) (end 31 40)) (probe (position 31 35))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind specialization) (ordinal 0) (authored-target "Array")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 32 26) (end 32 30)) (probe (position 32 26))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 32 16) (end 32 24)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 33 16) (end 33 26)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 9 35) (end 9 38)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind specialization) (ordinal 0) (authored-target "Bag")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 13 34) (end 13 38)) (probe (position 13 34))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind specialization) (ordinal 0) (authored-target "List")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 14 22) (end 14 29)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 14 12) (end 14 20)) (probe (position 14 12))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 25 38) (end 25 42)) (probe (position 25 38))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind specialization) (ordinal 0) (authored-target "List")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 26 26) (end 26 29)) (probe (position 26 26))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Set")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 26 16) (end 26 24)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 21 39) (end 21 49)) (probe (position 21 39))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind specialization) (ordinal 0) (authored-target "OrderedSet")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 17 32) (end 17 35)) (probe (position 17 32))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind specialization) (ordinal 0) (authored-target "Set")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 18 26) (end 18 32)) (probe (position 18 26))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_11_variable_length_collection_types.md") (range (start 18 16) (end 18 24)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/15_11_variable_length_collection_types.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unsupported)))
  )
)
~~~
