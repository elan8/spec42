# META
~~~ini
description=Issue 230: Reserved declaration and short names are malformed, quoted names remain resolved, and later siblings survive (SysML 8.2.2.1.2).
type=file
~~~
# SOURCE
~~~sysml
package ReservedNames {
    attribute port;
    ref from;
    ref to;
    attribute 'port';
    ref 'from';
    ref 'to';
    item <from> invalidShort;
    item <'from'> validShort;
    attribute retained;
    part def Base { attribute inherited; }
    part receiver : Base {
        attribute port :>> inherited;
        attribute :>> inherited;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_parser_names.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 1 14) (end 1 18))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 2 8) (end 2 12))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 3 8) (end 3 10))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 7 10) (end 7 14))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 12 18) (end 12 22))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue230_parser_names.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 1 14) (end 1 18))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 2 8) (end 2 12))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 3 8) (end 3 10))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 7 10) (end 7 14))
      )
      (diagnostic
        (severity error)
        (code "reserved_declaration_name")
        (source "parser")
        (range (start 12 18) (end 12 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:faf0b7c2d1efdaccd35f74368c08dc5a56ff981427e69aa1b2c0721cba63a3c1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (malformed-name (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (malformed-name (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (malformed-name (kind ref) (ordinal 1))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::from"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::invalidShort"))) (kind item) (membership (kind feature) (visibility default)) (facts (short-name malformed)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::port"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "inherited")))))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name "inherited") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "inherited")))))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::retained"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::to"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::validShort"))) (kind item) (membership (kind feature) (visibility default)) (facts (short-name "from")))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")))))
    (reference (id (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited"))) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")))
      (subtype (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited")))
      (featured-by (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")))
      (subtype (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver")))
      (type (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")) (source direct))
      (supertype (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver")))
      (supertype (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited")) (scopes any feature))
      (supertype (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver")))
      (supertype (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited")) (scopes any feature))
      (subtype (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/issue230_parser_names.md") (range (start 11 20) (end 11 24)) (probe (position 11 20))
    (reference (id (source (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::receiver"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base")))))
    )
  )
  (query (document "memory://snapshot/issue230_parser_names.md") (range (start 12 27) (end 12 36)) (probe (position 12 27))
    (reference (id (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (malformed-name (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/issue230_parser_names.md") (range (start 13 22) (end 13 31)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/issue230_parser_names.md") (path (named (kind package) (name "ReservedNames")) (named (kind part) (name "receiver")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "inherited")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue230_parser_names.md") (qualified-name "ReservedNames::Base::inherited")))))
    )
  )
)
~~~
