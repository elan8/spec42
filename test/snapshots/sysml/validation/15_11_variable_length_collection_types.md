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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwRef,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_11-Variable Length Collection Types''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Collections::*')
    (part_def 'SparePart')
    (part_def 'Person')
    (comment)
    (attribute_def ''Bag<SparePart>'' :> 'Bag'
      (part_usage ref :>> 'elements' : 'SparePart'))
    (attribute_def ''List<Integer>'' :> 'List'
      (default_ref_usage 'value' :>> 'elements' : 'Integer'))
    (attribute_def ''Set<String>'' :> 'Set'
      (attribute_usage :>> 'elements' : 'String'))
    (attribute_def ''OrderedSet<Person>'' :> 'OrderedSet'
      (part_usage ref :>> 'elements' : 'Person'))
    (attribute_def ''List<Set<Person>>'' :> 'List'
      (attribute_usage :>> 'elements' : 'Set'
        (part_usage ref :>> 'elements' : 'Person')))
    (attribute_def ''Array<Real>[4]'' :> 'Array'
      (attribute_usage :>> 'elements' : 'Real')
      (attribute_usage :>> 'dimensions' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Bag'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'String'
semantic.unresolved_name 'OrderedSet'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Bag'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'String'
semantic.unresolved_name 'OrderedSet'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "63bf74fa1f6c1e322ea2ca1b5d401cc15d3b91ddbf9cbcd63f6a80d187a1a2a1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (kind "package") (name "15_11-Variable Length Collection Types") (declared-name "15_11-Variable Length Collection Types") (range (start (line 0) (character 0)) (end (line 0) (character 810))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 27))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind "attribute def") (name "Array<Real>[4]") (declared-name "Array<Real>[4]") (range (start (line 31) (character 1)) (end (line 31) (character 109))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "Array") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (range (start (line 33) (character 2)) (end (line 33) (character 31))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions") (range (start (line 33) (character 16)) (end (line 33) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (range (start (line 32) (character 2)) (end (line 32) (character 31))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "elements") (range (start (line 32) (character 16)) (end (line 32) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind "attribute def") (name "Bag<SparePart>") (declared-name "Bag<SparePart>") (range (start (line 9) (character 1)) (end (line 9) (character 79))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "Bag") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind "ref") (name "") (range (start (line 10) (character 2)) (end (line 10) (character 35))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (authored (membership (kind Feature)) (relationships (typing (reference "SparePart") (range (start (line 10) (character 24)) (end (line 10) (character 34)))) (redefinition (reference "elements") (range (start (line 10) (character 15)) (end (line 10) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind "attribute def") (name "List<Integer>") (declared-name "List<Integer>") (range (start (line 13) (character 1)) (end (line 13) (character 74))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "List") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (range (start (line 14) (character 2)) (end (line 14) (character 30))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (redefinition (reference "elements") (range (start (line 14) (character 8)) (end (line 14) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind "attribute def") (name "List<Set<Person>>") (declared-name "List<Set<Person>>") (range (start (line 25) (character 1)) (end (line 25) (character 117))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "List") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (range (start (line 26) (character 2)) (end (line 26) (character 69))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (authored (membership (kind Feature)) (relationships (typing (reference "Set") (range none)) (redefinition (reference "elements") (range (start (line 26) (character 16)) (end (line 26) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind "attribute def") (name "OrderedSet<Person>") (declared-name "OrderedSet<Person>") (range (start (line 21) (character 1)) (end (line 21) (character 87))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "OrderedSet") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind "ref") (name "") (range (start (line 22) (character 2)) (end (line 22) (character 32))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person") (range (start (line 22) (character 24)) (end (line 22) (character 31)))) (redefinition (reference "elements") (range (start (line 22) (character 15)) (end (line 22) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person"))) (kind "part def") (name "Person") (declared-name "Person") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind "attribute def") (name "Set<String>") (declared-name "Set<String>") (range (start (line 17) (character 1)) (end (line 17) (character 74))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))) (authored (membership (kind Owning)) (relationships (typing (reference "Set") (range none)))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (range (start (line 18) (character 2)) (end (line 18) (character 33))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (redefinition (reference "elements") (range (start (line 18) (character 16)) (end (line 18) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart"))) (kind "part def") (name "SparePart") (declared-name "SparePart") (range (start (line 4) (character 1)) (end (line 4) (character 20))) (parent (node (document "d0") (qualified-name "15_11-Variable Length Collection Types"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Collections::*") (range (start (line 2) (character 16)) (end (line 2) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (range (start (line 33) (character 16)) (end (line 33) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 32) (character 16)) (end (line 32) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Array<Real>[4]::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>"))) (kind featureTyping) (ordinal 0)) (authored-target "Bag") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind featureTyping) (ordinal 0)) (authored-target "SparePart") (range (start (line 10) (character 24)) (end (line 10) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::SparePart")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Bag<SparePart>::"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 10) (character 15)) (end (line 10) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>"))) (kind featureTyping) (ordinal 0)) (authored-target "List") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 14) (character 8)) (end (line 14) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Integer>::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>"))) (kind featureTyping) (ordinal 0)) (authored-target "List") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Set") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 26) (character 16)) (end (line 26) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::List<Set<Person>>::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>"))) (kind featureTyping) (ordinal 0)) (authored-target "OrderedSet") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range (start (line 22) (character 24)) (end (line 22) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::OrderedSet<Person>::"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 22) (character 15)) (end (line 22) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>"))) (kind featureTyping) (ordinal 0)) (authored-target "Set") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 18) (character 16)) (end (line 18) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_11-Variable Length Collection Types::Set<String>::elements")))))
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
