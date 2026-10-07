# META
~~~ini
description=KerML Address Book: AddressBookModel
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
package AddressBookModel {
	
	class Entry {
		name: String;
		address: String;
	}
	
	class AddressBook {
		entries: Entry[*];
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/address_book_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 8) (end 4 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 11) (end 5 17))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bbdc68cb941921a81f4e3569939b23633d0b43602c76c41f952bebf8af6558b3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries"))) (kind default-reference) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Entry")))))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (kind kerml-literal-infinity) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::address"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::name"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries"))) (kind featureTyping) (ordinal 0))
      (authored-target "Entry")
      (outcome (status resolved) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")))))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::address"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::name"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries"))) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries"))) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::address"))) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::name"))) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries")))
      (featured-by (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook")))
      (type (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")) (provenance authored))
      (effective-type (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")) (source direct))
      (supertype (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook")))
    )
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook")))
    )
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/address_book_model.md") (path (named (kind package) (name "AddressBookModel")) (named (kind class-def) (name "AddressBook")) (named (kind default-reference) (name "entries")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-infinity) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")))
      (subtype (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::address")))
      (featured-by (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")))
    )
    (declaration (id (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::name")))
      (featured-by (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/address_book_model.md") (range (start 0 15) (end 0 30)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/address_book_model.md") (range (start 9 11) (end 9 16)) (probe (position 9 11))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::AddressBook::entries"))) (kind featureTyping) (ordinal 0) (authored-target "Entry")
      (outcome (status resolved) (target (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry")))))
    )
  )
  (query (document "memory://snapshot/address_book_model.md") (range (start 5 11) (end 5 17)) (probe (position 5 11))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::address"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/address_book_model.md") (range (start 4 8) (end 4 14)) (probe (position 4 8))
    (reference (id (source (node (document "memory://snapshot/address_book_model.md") (qualified-name "AddressBookModel::Entry::name"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
