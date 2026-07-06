//! SysML v2 reserved keywords and keyword documentation for completion/hover.

/// SysML v2 / KerML reserved keywords (BNF 8.2.2.1.2 RESERVED_KEYWORD, plus grammar extensions:
/// value, provides, requires).
/// Single source of truth for semantic token fallback and keyword checks (goto-def, rename).
/// Note: "position" is a contextual keyword (position_statement) only, not reserved—valid as identifier.
pub const RESERVED_KEYWORDS: &[&str] = &[
    "about",
    "abstract",
    "accept",
    "action",
    "actor",
    "after",
    "alias",
    "all",
    "allocate",
    "allocation",
    "analysis",
    "and",
    "as",
    "assert",
    "assign",
    "assume",
    "at",
    "attribute",
    "bind",
    "binding",
    "by",
    "calc",
    "case",
    "comment",
    "concern",
    "connect",
    "connection",
    "constant",
    "constraint",
    "crosses",
    "decide",
    "def",
    "default",
    "defined",
    "dependency",
    "derived",
    "do",
    "doc",
    "else",
    "end",
    "entry",
    "enum",
    "event",
    "exhibit",
    "exit",
    "expose",
    "false",
    "filter",
    "first",
    "flow",
    "for",
    "fork",
    "frame",
    "from",
    "hastype",
    "if",
    "implies",
    "import",
    "in",
    "include",
    "individual",
    "inout",
    "interface",
    "istype",
    "item",
    "join",
    "language",
    "library",
    "locale",
    "loop",
    "merge",
    "message",
    "meta",
    "metadata",
    "nonunique",
    "not",
    "null",
    "objective",
    "occurrence",
    "of",
    "or",
    "ordered",
    "out",
    "package",
    "parallel",
    "part",
    "perform",
    "port",
    "private",
    "protected",
    "provides",
    "public",
    "redefines",
    "ref",
    "references",
    "render",
    "rendering",
    "rep",
    "require",
    "requirement",
    "requires",
    "return",
    "satisfy",
    "send",
    "snapshot",
    "specializes",
    "stakeholder",
    "standard",
    "state",
    "subject",
    "subsets",
    "succession",
    "terminate",
    "then",
    "timeslice",
    "to",
    "transition",
    "true",
    "until",
    "use",
    "value",
    "variant",
    "variation",
    "verification",
    "verify",
    "via",
    "view",
    "viewpoint",
    "when",
    "while",
    "xor",
];

/// Returns true if the word is a SysML v2 reserved keyword.
pub fn is_reserved_keyword(word: &str) -> bool {
    RESERVED_KEYWORDS.contains(&word)
}

/// Curated subset of reserved keywords used for completion suggestions and hover docs.
pub fn sysml_keywords() -> &'static [&'static str] {
    &[
        "package",
        "library",
        "part",
        "attribute",
        "port",
        "connection",
        "interface",
        "item",
        "value",
        "action",
        "requirement",
        "ref",
        "in",
        "out",
        "provides",
        "requires",
        "bind",
        "allocate",
        "abstract",
        "def",
        "variant",
        "references",
        "private",
        "public",
        "entry",
        "exit",
        "state",
        "do",
        "then",
        "transition",
        "constraint",
        "exhibit",
    ]
}

/// Short documentation for a keyword. Returns None if unknown.
pub fn keyword_doc(keyword: &str) -> Option<&'static str> {
    let doc = match keyword {
        "package" => "Package: namespace for members (parts, actions, etc.).",
        "part" => "Part: structural element; can be definition (part def) or usage.",
        "attribute" => "Attribute: property with optional type and default.",
        "port" => "Port: interaction point (e.g. for connections).",
        "connection" => "Connection: links between ports.",
        "interface" => "Interface: contract for ports.",
        "action" => "Action: behavior definition or usage.",
        "requirement" => "Requirement: requirement definition or usage.",
        "ref" => "Ref: reference to an element (e.g. ref action, ref individual).",
        "in" | "out" => "In/out: input or output (e.g. in action, in attribute).",
        "provides" => "Provides: part provides a capability (e.g. Execution = MCU).",
        "requires" => "Requires: part requires a capability.",
        "bind" => "Bind: bind logical port to physical port.",
        "allocate" => "Allocate: allocate logical to physical (e.g. allocate x to y).",
        "abstract" => "Abstract: abstract part or element.",
        "def" => "Def: definition (e.g. part def, attribute def).",
        "variant" => "Variant: variant part.",
        "library" => "Library: library package.",
        "value" => "Value: value definition or usage.",
        "item" => "Item: item definition or usage.",
        "references" => "References: requirement references.",
        "private" | "public" => "Visibility: private or public.",
        "entry" => "Entry: entry action or behavior when entering a state.",
        "exit" => "Exit: exit action or behavior when leaving a state.",
        "state" => "State: state definition or usage in a state machine.",
        "do" => "Do: activity performed while in a state.",
        "then" => "Then: target state or action in a transition.",
        "transition" => "Transition: transition between states.",
        "constraint" => "Constraint: invariant or constraint block.",
        "exhibit" => "Exhibit: exhibit state machine (e.g. exhibit state name { }).",
        _ => return None,
    };
    Some(doc)
}

/// Returns Markdown string for keyword hover (bold keyword, description, optional syntax hint).
/// Covers every word in [`RESERVED_KEYWORDS`] — see the completeness test at the bottom of this
/// module — so hover never comes up empty for a real SysML v2/KerML reserved word. `None` is
/// only returned for words that aren't reserved at all.
pub fn keyword_hover_markdown(keyword: &str) -> Option<String> {
    let (desc, syntax): (&str, Option<&str>) = match keyword {
        "package" => (
            "Namespace for members (parts, actions, etc.).",
            Some("`package name { }`"),
        ),
        "part" => (
            "Structural element; can be definition (part def) or usage.",
            Some("`part def Name : Type;` or `part name : Type;`"),
        ),
        "attribute" => (
            "Property with optional type and default.",
            Some("`attribute def name : Type;`"),
        ),
        "port" => (
            "Interaction point (e.g. for connections).",
            Some("`port def name : Interface;`"),
        ),
        "connection" => ("Links between ports.", Some("`connection name (a, b);`")),
        "connect" => (
            "Statement form of a connection usage, binding two feature ends.",
            Some("`connect a to b;`"),
        ),
        "interface" => ("Contract for ports.", Some("`interface def name { }`")),
        "action" => ("Behavior definition or usage.", Some("`action def name;`")),
        "requirement" => (
            "Requirement definition or usage.",
            Some("`requirement def name;`"),
        ),
        "ref" => (
            "Reference to an element (e.g. ref action, ref individual).",
            Some("`ref name;`"),
        ),
        "in" | "out" => (
            "Input or output (e.g. in action, in attribute).",
            Some("`in name : Type;`"),
        ),
        "inout" => (
            "Bidirectional parameter direction (both input and output).",
            Some("`inout name : Type;`"),
        ),
        "provides" => (
            "Part provides a capability.",
            Some("`provides name = value;`"),
        ),
        "requires" => (
            "Part requires a capability.",
            Some("`requires name = value;`"),
        ),
        "bind" => (
            "Bind logical port to physical port.",
            Some("`bind a to b;`"),
        ),
        "binding" => (
            "Binding connector usage kind: asserts two features always have the same value.",
            Some("`binding a = b;`"),
        ),
        "allocate" => ("Allocate logical to physical.", Some("`allocate x to y;`")),
        "allocation" => (
            "Allocation definition/usage relating logical structure/behavior to physical structure/behavior.",
            Some("`allocation def Name;`"),
        ),
        "abstract" => (
            "Abstract part or element: cannot be instantiated directly, only specialized.",
            Some("`abstract part def Name;`"),
        ),
        "def" => (
            "Definition (e.g. part def, attribute def).",
            Some("`part def`, `attribute def`, etc."),
        ),
        "variant" => (
            "Variant member of a `variation` definition/usage: one of its allowed choices.",
            Some("`variant name;` or `variant part name : Type;`"),
        ),
        "variation" => (
            "Marks a definition/usage as a variation point whose members are all `variant`s.",
            Some("`variation part def Name { variant a; variant b; }`"),
        ),
        "library" => ("Library package.", Some("`library package name { }`")),
        "standard" => (
            "Marks a library package as part of the standard (built-in) model library.",
            Some("`standard library package Name { }`"),
        ),
        "value" | "item" => (keyword_doc(keyword)?, None),
        "references" => (
            "Requirement/case reference: an element the requirement/case depends on but doesn't own.",
            Some("`references name : Type;`"),
        ),
        "private" | "public" | "protected" => (
            "Visibility modifier controlling whether a member is exported from its namespace.",
            None,
        ),
        "entry" => (
            "Entry action or behavior when entering a state.",
            Some("`entry action name;`"),
        ),
        "exit" => (
            "Exit action or behavior when leaving a state.",
            Some("`exit action name;`"),
        ),
        "state" => (
            "State definition or usage in a state machine.",
            Some("`state name { }`"),
        ),
        "do" => (
            "Activity performed while in a state.",
            Some("`do action name;`"),
        ),
        "then" => (
            "Target state or action in a transition, or the successor in a succession.",
            Some("`transition ev then target;` or `first a then b;`"),
        ),
        "transition" => (
            "Transition between states.",
            Some("`transition event then target;`"),
        ),
        "constraint" => ("Invariant or constraint block.", None),
        "exhibit" => ("Exhibit state machine.", Some("`exhibit state name { }`")),
        "enum" => (
            "Enumeration definition: a variation whose values are its variants.",
            Some("`enum def Name { enum a; enum b; }`"),
        ),
        "occurrence" => (
            "Base kind for anything that can occur in time (the root of parts, actions, states, etc.).",
            Some("`occurrence def Name;`"),
        ),
        "individual" => (
            "Marks an occurrence usage/definition as representing a single, non-repeating occurrence.",
            Some("`individual part name;`"),
        ),
        "event" => (
            "Event occurrence usage: marks the instant at which something happens.",
            Some("`event occurrence name;`"),
        ),
        "snapshot" => (
            "Snapshot usage: an occurrence representing an instantaneous state of its type.",
            Some("`snapshot name;`"),
        ),
        "timeslice" => (
            "Timeslice usage: an occurrence representing a time-bounded portion of its type.",
            Some("`timeslice name;`"),
        ),
        "calc" => (
            "Calculation definition/usage: computes a return value from its parameters.",
            Some("`calc def name(x : Type) : Type { return x; }`"),
        ),
        "case" => (
            "Base kind shared by analysis/verification/use cases.",
            Some("`case def Name;`"),
        ),
        "analysis" => (
            "Analysis case definition/usage: evaluates a calculation over a subject.",
            Some("`analysis def Name;`"),
        ),
        "verification" => (
            "Verification case definition/usage: verifies that a requirement is satisfied.",
            Some("`verification def Name;`"),
        ),
        "verify" => (
            "Requirement verification usage inside a verification case.",
            Some("`verify requirement req;` or `verify req;`"),
        ),
        "use" => (
            "Prefix for `use case` definitions/usages.",
            Some("`use case def Name;`"),
        ),
        "view" => (
            "View definition/usage: renders a viewpoint over the model.",
            Some("`view def Name;`"),
        ),
        "viewpoint" => (
            "Viewpoint definition/usage: specifies stakeholder concerns a view must address.",
            Some("`viewpoint def Name;`"),
        ),
        "rendering" => (
            "Rendering definition/usage: produces a concrete visual/textual representation.",
            Some("`rendering def Name;`"),
        ),
        "render" => (
            "Statement that renders a view via a rendering.",
            Some("`render name;`"),
        ),
        "expose" => (
            "View expose statement: exposes members into a view through a viewpoint.",
            Some("`expose Pkg::*;`"),
        ),
        "metadata" => (
            "Metadata definition/usage: annotates elements with semantic tags.",
            Some("`metadata def Name;` / `@Name`"),
        ),
        "meta" => (
            "Metaclass-cast operator: treats an element as an instance of its own metaclass.",
            Some("`x meta Type`"),
        ),
        "concern" => (
            "Concern usage: captures a stakeholder's issue of interest addressed by a view.",
            Some("`concern name;`"),
        ),
        "stakeholder" => (
            "Viewpoint stakeholder parameter.",
            Some("`stakeholder name : Type;`"),
        ),
        "objective" => (
            "Case objective: states what an analysis/verification/use case sets out to achieve.",
            Some("`objective { ... }`"),
        ),
        "subject" => (
            "Requirement/case subject: the element the requirement/case is about.",
            Some("`subject name : Type;`"),
        ),
        "actor" => (
            "Use case actor: an external party interacting with the subject system.",
            Some("`actor name : Type;`"),
        ),
        "include" => (
            "Use case inclusion: one use case includes another as part of its behavior.",
            Some("`include useCase name;`"),
        ),
        "frame" => (
            "Requirement frame concern reference.",
            Some("`frame name;`"),
        ),
        "filter" => (
            "Import filter: restricts an imported namespace to members matching a condition.",
            Some("`import Pkg::* [condition];`"),
        ),
        "dependency" => (
            "Dependency relationship: one or more elements depend on one or more others.",
            Some("`dependency from a to b;`"),
        ),
        "alias" => (
            "Alias member: an alternate name for another member.",
            Some("`alias <shortName> name for Target;`"),
        ),
        "import" => (
            "Imports members of another namespace into the current one.",
            Some("`import Pkg::*;` / `import all Pkg::*;`"),
        ),
        "all" => (
            "Modifier on `import` that also imports otherwise-private members.",
            Some("`import all Pkg::*;`"),
        ),
        "flow" => (
            "Flow: item transfer between features over time.",
            Some("`flow source to target;`"),
        ),
        "message" => (
            "Message: a flow of a payload that triggers a behavior at its target.",
            Some("`message name from a to b;`"),
        ),
        "succession" => (
            "Succession: an ordering relationship between occurrences.",
            Some("`first a then b;`"),
        ),
        "first" => (
            "First: the source occurrence in a `first ... then ...` succession statement.",
            Some("`first a then b;`"),
        ),
        "via" => (
            "Specifies the port a transition trigger, `accept`, or `send` goes through.",
            Some("`accept event via port;`"),
        ),
        "send" => (
            "Sends a payload to a target, optionally via a port.",
            Some("`send payload to target via port;`"),
        ),
        "accept" => (
            "Accepts an incoming payload/event, optionally via a specific port.",
            Some("`accept payload via port;`"),
        ),
        "perform" => (
            "Enacts an action within a structure or another behavior.",
            Some("`perform action name;`"),
        ),
        "fork" => (
            "Control node that splits execution into concurrent flows.",
            Some("`fork; then a; then b;`"),
        ),
        "join" => (
            "Control node that synchronizes concurrent flows.",
            Some("`join; then next;`"),
        ),
        "merge" => (
            "Control node that combines alternative incoming flows.",
            Some("`merge; then next;`"),
        ),
        "decide" => (
            "Control node that branches execution based on conditions.",
            Some("`decide; then if cond a; else b;`"),
        ),
        "if" => (
            "Conditional branch in an action body.",
            Some("`if cond a; else b;`"),
        ),
        "else" => (
            "Alternative branch taken when an `if` condition is false.",
            Some("`if cond a; else b;`"),
        ),
        "when" => (
            "Condition-based transition trigger, evaluated continuously rather than on an event.",
            Some("`transition when cond then target;`"),
        ),
        "while" => (
            "Loop that repeats its body while a condition holds.",
            Some("`while cond loop body;`"),
        ),
        "loop" => (
            "Repeats an action body; paired with `while` or `for`.",
            Some("`while cond loop body;`"),
        ),
        "for" => (
            "Loop that iterates over a collection.",
            Some("`for x in collection loop body;`"),
        ),
        "until" => (
            "Loop-termination condition, checked after the body.",
            Some("`loop body until cond;`"),
        ),
        "assign" => (
            "Assigns a value to a feature during an action.",
            Some("`assign target := value;`"),
        ),
        "terminate" => (
            "Immediately ends an occurrence.",
            Some("`terminate name;`"),
        ),
        "return" => (
            "Return parameter/value of a calculation, function, or action.",
            Some("`return name : Type;`"),
        ),
        "assert" => (
            "Asserts that a constraint holds (or, with `not`, that it fails).",
            Some("`assert constraint { expr }`"),
        ),
        "assume" => (
            "Assumption constraint usage: a condition taken as given rather than checked.",
            Some("`assume constraint { expr }`"),
        ),
        "satisfy" => (
            "Asserts a usage satisfies a requirement.",
            Some("`satisfy req by subject;`"),
        ),
        "require" => (
            "Requirement's evaluable condition.",
            Some("`require constraint { expr }`"),
        ),
        "crosses" => (
            "Relates a connector/port end to a feature on the far side it crosses into.",
            Some("`end part hub : Hub crosses device.connectingHub;`"),
        ),
        "specializes" => (
            "Declares a definition/usage as a specialization of another (`:>`).",
            Some("`part def Sub :> Super;`"),
        ),
        "redefines" => (
            "Redefines an inherited feature (`:>>`).",
            Some("`part name :>> inherited;`"),
        ),
        "subsets" => (
            "Declares a feature as a subset of another feature (`:>`).",
            Some("`part name :> superset;`"),
        ),
        "defined" => (
            "Introduces `defined by`, an alternative to `:` for typing a declaration.",
            Some("`name defined by Type;`"),
        ),
        "constant" => (
            "Feature modifier asserting the feature's value never changes over time.",
            Some("`attribute name : Type constant;`"),
        ),
        "derived" => (
            "Feature modifier indicating the feature's value is computed rather than stored.",
            Some("`attribute name : Type derived;`"),
        ),
        "ordered" => (
            "Feature modifier: the feature's multiple values have a significant order.",
            Some("`part items : Item[*] ordered;`"),
        ),
        "nonunique" => (
            "Feature modifier: the feature may hold duplicate values.",
            Some("`part items : Item[*] nonunique;`"),
        ),
        "default" => (
            "Introduces a default (overridable) feature value, as opposed to a fixed `=` value.",
            Some("`attribute name : Type default 0;`"),
        ),
        "end" => (
            "Marks a feature as an end (connection point) of a connector-like definition/usage.",
            Some("`end producer : Type;`"),
        ),
        "parallel" => (
            "Marks a state as parallel: its substates execute concurrently with no transitions between them.",
            Some("`state parallel Name { ... }`"),
        ),
        "as" => (
            "Introduces an alias name in an import, or casts an expression to a type.",
            Some("`import Pkg as P;` / `expr as Type`"),
        ),
        "istype" => (
            "Type-test operator: true if the value's type matches the given type exactly.",
            Some("`x istype Type`"),
        ),
        "hastype" => (
            "Type-test operator: true if the value has the given type among its types.",
            Some("`x hastype Type`"),
        ),
        "and" => ("Logical AND operator in expressions.", Some("`a and b`")),
        "or" => ("Logical OR operator in expressions.", Some("`a or b`")),
        "not" => ("Logical NOT operator in expressions.", Some("`not a`")),
        "implies" => (
            "Logical implication operator in expressions.",
            Some("`a implies b`"),
        ),
        "xor" => (
            "Logical exclusive-or operator in expressions.",
            Some("`a xor b`"),
        ),
        "true" => ("Boolean literal.", None),
        "false" => ("Boolean literal.", None),
        "null" => ("Literal representing the absence of a value.", None),
        "of" => (
            "Introduces the payload feature carried by a `flow` or `message`.",
            Some("`flow source to target of payloadFeature;`"),
        ),
        "to" => (
            "Introduces the target endpoint of a `flow`, `connect`, `allocate`, or `dependency`.",
            Some("`flow a to b;`"),
        ),
        "from" => (
            "Introduces the source/client side of a `dependency`.",
            Some("`dependency from a to b;`"),
        ),
        "by" => (
            "Introduces the subject of a `satisfy`, or pairs with `defined`/`typed` (`defined by`, `typed by`).",
            Some("`satisfy req by subject;`"),
        ),
        "at" => (
            "Occurrence-timing keyword used in temporal expressions.",
            None,
        ),
        "after" => (
            "Occurrence-timing keyword: relates one occurrence's timing to another's.",
            None,
        ),
        "comment" => (
            "Comment annotation, optionally naming the elements it's `about`.",
            Some("`comment about Target /* text */`"),
        ),
        "about" => (
            "Names the elements a `comment` annotates.",
            Some("`comment about Target /* text */`"),
        ),
        "doc" => (
            "Documentation attached to the enclosing element.",
            Some("`doc /* text */`"),
        ),
        "rep" => (
            "Textual representation of an element in another notation.",
            Some("`rep language \"OCL\" /* expr */`"),
        ),
        "language" => (
            "Names the notation a `rep` (textual representation) body is written in.",
            Some("`language \"OCL\"`"),
        ),
        "locale" => (
            "Optional locale tag on a `comment` or `doc`.",
            Some("`doc locale \"en-US\" /* text */`"),
        ),
        _ => return None,
    };
    let mut md = format!("**{}**\n\n{}", keyword, desc);
    if let Some(syn) = syntax {
        md.push_str(&format!("\n\nSyntax: {}", syn));
    }
    md.push_str("\n\n*See SysML v2 specification for full syntax.*");
    Some(md)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Enforces the single-source-of-truth claim in [`keyword_hover_markdown`]'s doc comment:
    /// every reserved keyword must have hover documentation, so adding a new keyword to
    /// `RESERVED_KEYWORDS` without also documenting it here fails the build instead of silently
    /// producing empty hover for that keyword.
    #[test]
    fn every_reserved_keyword_has_hover_markdown() {
        let missing: Vec<&str> = RESERVED_KEYWORDS
            .iter()
            .copied()
            .filter(|kw| keyword_hover_markdown(kw).is_none())
            .collect();
        assert!(
            missing.is_empty(),
            "reserved keywords missing hover markdown: {missing:?}"
        );
    }
}
