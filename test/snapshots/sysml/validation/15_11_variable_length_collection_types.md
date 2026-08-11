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
  (document "15_11_variable_length_collection_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 1) (end 9 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 15) (end 10 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 1) (end 13 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 1) (end 17 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 1) (end 21 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 15) (end 22 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 1) (end 25 117))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 2) (end 26 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 1) (end 31 109))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 2) (end 32 31))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "eadbe5317da8801e148af1966bdf45457ba347ea16deb5b85fd73f0ad9dae010") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (kind "package") (name "15_11-Variable Length Collection Types") (declared-name "15_11-Variable Length Collection Types"))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind "attribute def") (name "Array<Real>[4]") (declared-name "Array<Real>[4]") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "Array")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind "attribute def") (name "Bag<SparePart>") (declared-name "Bag<SparePart>") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "Bag")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (authored (membership (kind Feature)) (relationships (typing (reference "SparePart")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind "attribute def") (name "List<Integer>") (declared-name "List<Integer>") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "List")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind "attribute def") (name "List<Set<Person>>") (declared-name "List<Set<Person>>") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "List")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (authored (membership (kind Feature)) (relationships (typing (reference "Set")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind "attribute def") (name "OrderedSet<Person>") (declared-name "OrderedSet<Person>") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "OrderedSet")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind "ref") (name "") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person"))) (kind "part def") (name "Person") (declared-name "Person") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind "attribute def") (name "Set<String>") (declared-name "Set<String>") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "Set")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart"))) (kind "part def") (name "SparePart") (declared-name "SparePart") (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Collections::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind featureTyping) (ordinal 0)) (authored-target "Bag") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind featureTyping) (ordinal 0)) (authored-target "SparePart") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind featureTyping) (ordinal 0)) (authored-target "List") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind featureTyping) (ordinal 0)) (authored-target "List") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Set") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind featureTyping) (ordinal 0)) (authored-target "OrderedSet") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind featureTyping) (ordinal 0)) (authored-target "Set") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 24) (end 22 31)) (probe (position 22 24))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))
        (kind featureTyping) (ordinal 0) (authored-target "Person")
        (range (start 22 24) (end 22 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person") (range (start 5 1) (end 5 17)))
        )
      )
    )
    (query (range (start 10 15) (end 10 23)) (probe (position 10 15))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 10 15) (end 10 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 24)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 18 16) (end 18 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements") (range (start 18 2) (end 18 33)))
        )
      )
    )
    (query (range (start 22 15) (end 22 23)) (probe (position 22 15))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 22 15) (end 22 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 16) (end 26 24)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 26 16) (end 26 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements") (range (start 26 2) (end 26 69)))
        )
      )
    )
    (query (range (start 32 16) (end 32 24)) (probe (position 32 16))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 32 16) (end 32 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements") (range (start 32 2) (end 32 31)))
        )
      )
    )
    (query (range (start 10 24) (end 10 34)) (probe (position 10 24))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))
        (kind featureTyping) (ordinal 0) (authored-target "SparePart")
        (range (start 10 24) (end 10 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart") (range (start 4 1) (end 4 20)))
        )
      )
    )
    (query (range (start 33 16) (end 33 26)) (probe (position 33 16))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))
        (kind redefinition) (ordinal 0) (authored-target "dimensions")
        (range (start 33 16) (end 33 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions") (range (start 33 2) (end 33 31)))
        )
      )
    )
    (query (range (start 2 16) (end 2 27)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Collections::*")
        (range (start 2 16) (end 2 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 8) (end 14 20)) (probe (position 14 8))
      (reference
        (source (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 14 8) (end 14 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements") (range (start 14 2) (end 14 30)))
        )
      )
    )
  )
)
~~~
