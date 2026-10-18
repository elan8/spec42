# META
~~~ini
description=Issue 297: an extended `end #q name;` takes the positional end ordinal a plain `end name;` would
type=file
~~~
# SOURCE
~~~sysml
package ExtendedEnds {
    metadata def Q;
    part def Pump;
    part def Tank;

    // Plain and extended ends share one positional sequence. A typed `end #Q x : T;` is a plain
    // end declaration; the untyped `end #Q x;` is the ExtendedUsage spelling.
    connection def Feed {
        end supply : Pump;
        end #Q reservoir;
    }

    // A connection usage body is a part usage body. Connection bodies read `end #Q x;` as a plain
    // end; allocation and part bodies read it as the ExtendedUsage spelling.
    connection feed {
        end #Q supply;
        end reservoir : Tank;
    }

    allocation def Allocate {
        end supply : Pump;
        end #Q reservoir;
    }

    allocation pumping {
        end #Q supply;
        end reservoir : Tank;
    }

    part host : Pump {
        end #Q left;
        end right : Pump;
    }

    occurrence def Window {
        occurrence span {
            end #Q opening;
            end #Q closing;
        }
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_extended_end_positional_ends.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0fd5e254254ce9d4c453b95ec25be597d68bd4094fc556dfa897e6170d47ddd8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::reservoir"))) (kind extended-usage) (membership (kind feature) (visibility default)) (facts (modifiers end) (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocation-def) (name "Allocate")) (named (kind extended-usage) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::reservoir"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection-def) (name "Feed")) (named (kind connection) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::closing"))) (kind extended-usage) (membership (kind feature) (visibility default)) (facts (modifiers end) (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "closing")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::opening"))) (kind extended-usage) (membership (kind feature) (visibility default)) (facts (modifiers end) (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "opening")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::supply"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection) (name "feed")) (named (kind connection) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::left"))) (kind extended-usage) (membership (kind feature) (visibility default)) (facts (modifiers end) (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind part) (name "host")) (named (kind extended-usage) (name "left")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pump")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping"))) (kind allocate) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tank")))))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::supply"))) (kind extended-usage) (membership (kind feature) (visibility default)) (facts (modifiers end) (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocate) (name "pumping")) (named (kind extended-usage) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (metadataAnnotation (reference "Q")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocation-def) (name "Allocate")) (named (kind extended-usage) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection-def) (name "Feed")) (named (kind connection) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "closing")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "opening")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection) (name "feed")) (named (kind connection) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind part) (name "host")) (named (kind extended-usage) (name "left")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")))))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocate) (name "pumping")) (named (kind extended-usage) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocation-def) (name "Allocate")) (named (kind extended-usage) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocation-def) (name "Allocate")) (named (kind extended-usage) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection-def) (name "Feed")) (named (kind connection) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection-def) (name "Feed")) (named (kind connection) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "closing")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "closing")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "opening")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "opening")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection) (name "feed")) (named (kind connection) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection) (name "feed")) (named (kind connection) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind part) (name "host")) (named (kind extended-usage) (name "left")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind part) (name "host")) (named (kind extended-usage) (name "left")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocate) (name "pumping")) (named (kind extended-usage) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocate) (name "pumping")) (named (kind extended-usage) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::reservoir"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::reservoir"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::closing"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::opening"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::supply"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::left"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::supply"))) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::reservoir")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate")))
      (type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::reservoir")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed")))
      (type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))
      (subtype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")))
      (subtype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir")) (scopes any))
      (subtype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::closing")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::opening")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed")))
      (type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::supply")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host")))
      (positional-ends (authored 2) (effective 2))
      (type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::left")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host")))
      (type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping")))
      (type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")) (source direct))
      (supertype (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::supply")))
      (featured-by (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping")))
    )
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::reservoir"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::reservoir"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::closing"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Window::span::opening"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::supply"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::left"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
  (annotation (element (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::supply"))) (form prefix-keyword) (definition (resolved (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed"))) (kind connection) (end (name (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) unconnected) (end (name (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::reservoir"))) unconnected))
  (connector (id (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed"))) (kind connection) (end (name (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::supply"))) unconnected) (end (name (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) unconnected))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 21 13) (end 21 14)) (probe (position 21 13))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocation-def) (name "Allocate")) (named (kind extended-usage) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 20 21) (end 20 25)) (probe (position 20 21))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Allocate::supply"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 9 13) (end 9 14)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection-def) (name "Feed")) (named (kind connection) (name "reservoir")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 8 21) (end 8 25)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Feed::supply"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 37 17) (end 37 18)) (probe (position 37 17))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "closing")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 36 17) (end 36 18)) (probe (position 36 17))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind occurrence-def) (name "Window")) (named (kind occurrence) (name "span")) (named (kind extended-usage) (name "opening")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 16 24) (end 16 28)) (probe (position 16 24))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::feed::reservoir"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 15 13) (end 15 14)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind connection) (name "feed")) (named (kind connection) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 29 16) (end 29 20)) (probe (position 29 16))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 30 13) (end 30 14)) (probe (position 30 13))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind part) (name "host")) (named (kind extended-usage) (name "left")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 31 20) (end 31 24)) (probe (position 31 20))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::host::right"))) (kind featureTyping) (ordinal 0) (authored-target "Pump")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Pump")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 26 24) (end 26 28)) (probe (position 26 24))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::pumping::reservoir"))) (kind featureTyping) (ordinal 0) (authored-target "Tank")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Tank")))))
    )
  )
  (query (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (range (start 25 13) (end 25 14)) (probe (position 25 13))
    (reference (id (source (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (path (named (kind package) (name "ExtendedEnds")) (named (kind allocate) (name "pumping")) (named (kind extended-usage) (name "supply")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Q")
      (outcome (status resolved) (target (node (document "memory://snapshot/parser_284_extended_end_positional_ends.md") (qualified-name "ExtendedEnds::Q")))))
    )
  )
)
~~~
