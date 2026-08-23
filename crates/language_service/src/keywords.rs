//! SysML v2 reserved keywords and keyword documentation for completion/hover.

/// SysML v2 reserved keywords from Language Specification 2.0, 8.2.2.1.2, owned by the syntax
/// service; re-exported so hosts that reach keywords through this crate keep one vocabulary.
pub use sysml_query::syntax::{is_reserved_keyword, RESERVED_KEYWORDS};

/// Normative reserved-word sequence from OMG SysML 2.0 Part 1,
/// 8.2.2.1.2, "Reserved Keywords".
///
/// Keep this independent of [`RESERVED_KEYWORDS`]: the test below detects additions,
/// omissions, and accidental reordering against the published language grammar.
#[cfg(test)]
const OMG_SYSML_V2_0_RESERVED_KEYWORDS: &str = "\
about abstract accept action actor after alias all allocate allocation analysis and as assert \
assign assume at attribute bind binding by calc case comment concern connect connection constant \
constraint crosses decide def default defined dependency derived do doc else end entry enum event \
exhibit exit expose false filter first flow for fork frame from hastype if implies import in include \
individual inout interface istype item join language library locale loop merge message meta metadata \
nonunique not null objective occurrence of or ordered out package parallel part perform port private \
protected public redefines ref references render rendering rep require requirement return satisfy send \
snapshot specializes stakeholder standard state subject subsets succession terminate then timeslice to \
transition true until use variant variation verification verify via view viewpoint when while xor";

/// Curated subset of reserved keywords used for completion suggestions.
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
        "action",
        "requirement",
        "ref",
        "in",
        "out",
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
    keyword_help(keyword).map(|help| help.description)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeywordHelp {
    pub description: &'static str,
    pub syntax: Option<&'static str>,
}

pub fn keyword_help(keyword: &str) -> Option<KeywordHelp> {
    let (description, syntax) = match keyword {
        "package" => (
            "Declares a package namespace that owns or imports model elements.",
            Some("`package Name { ... }`"),
        ),
        "part" => (
            "Part definition or usage. A definition classifies reusable kinds of parts; a usage represents a part in a context.",
            Some("`part def Vehicle :> BaseVehicle;` or `part vehicle : Vehicle;`"),
        ),
        "attribute" => (
            "Attribute definition or usage for data-valued features.",
            Some("`attribute def Temperature :> ScalarValues::Real;` or `attribute temperature : Temperature;`"),
        ),
        "port" => (
            "Port definition or usage describing an interaction point. A usage can be typed by a port definition or its conjugate.",
            Some("`port def FuelPort { in item fuel; }` or `port inlet : ~FuelPort;`"),
        ),
        "connection" => (
            "Connection definition or usage relating connector ends.",
            Some("`connection def Link;` or `connection link connect a to b;`"),
        ),
        "connect" => (
            "Shorthand connection usage relating two or more connector ends.",
            Some("`connect a to b;`"),
        ),
        "interface" => (
            "Interface definition or usage: a connection whose ends are ports.",
            Some("`interface def Link { end port source : P; end port target : ~P; }`"),
        ),
        "action" => (
            "Action definition or usage for behavior that occurs over time.",
            Some("`action def Process;` or `action process : Process;`"),
        ),
        "requirement" => (
            "Requirement definition or usage: a constraint with a subject and optional assumptions and required constraints.",
            Some("`requirement def Limit { subject system; require constraint { ... } }`"),
        ),
        "ref" => (
            "Makes a usage referential rather than owning/composite.",
            Some("`ref part driver : Person;`"),
        ),
        "in" | "out" => (
            "Direction on a usage or parameter. `in` supplies a value to its owner; `out` supplies a value from its owner.",
            Some("`in item command : Command;` or `out item status : Status;`"),
        ),
        "inout" => (
            "Bidirectional parameter direction (both input and output).",
            Some("`inout name : Type;`"),
        ),
        "bind" => (
            "Binding connector statement asserting that two features have the same value.",
            Some("`bind a = b;`"),
        ),
        "binding" => (
            "Binding connector usage kind: asserts two features always have the same value.",
            Some("`binding a = b;`"),
        ),
        "allocation" => (
            "Allocation definition or usage relating arbitrary usages, often across architecture viewpoints.",
            Some("`allocation def Mapping;` or `allocation mapping allocate source to target;`"),
        ),
        "allocate" => (
            "Shorthand allocation usage relating a source usage to a target usage.",
            Some("`allocate source to target;`"),
        ),
        "abstract" => (
            "Marks a definition or usage as abstract, so it serves as a general element rather than a concrete instance.",
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
        "library" => (
            "Marks a package as a reusable library package.",
            Some("`library package Name { ... }`"),
        ),
        "standard" => (
            "Marks a library package as part of the standard (built-in) model library.",
            Some("`standard library package Name { }`"),
        ),
        "item" => (
            "Item definition or usage for things that may flow, including data, matter, or energy.",
            Some("`item def Payload;` or `item payload : Payload;`"),
        ),
        "references" => (
            "Textual form of reference subsetting (`::>`): a feature refers to an existing feature.",
            Some("`ref part selected references availablePart;`"),
        ),
        "private" | "public" | "protected" => (
            "Visibility indicator controlling access to a namespace membership or import.",
            None,
        ),
        "entry" => (
            "State subaction performed when the state is entered.",
            Some("`entry action initialize;`"),
        ),
        "exit" => (
            "State subaction performed when the state is exited.",
            Some("`exit action cleanup;`"),
        ),
        "state" => (
            "State definition or usage. Its body can own entry, do, exit, and transition members.",
            Some("`state def OperatingStates { state idle; }` or `state operating : OperatingStates;`"),
        ),
        "do" => (
            "Introduces a state do-action or a transition effect.",
            Some("`do action operate;`"),
        ),
        "then" => (
            "Introduces the target of a transition or the successor end of a succession.",
            Some("`accept Signal then target;` or `first a then b;`"),
        ),
        "transition" => (
            "Explicit transition usage with a source, optional trigger/guard/effect, and target.",
            Some("`transition first source accept Signal if guard then target;`"),
        ),
        "constraint" => (
            "Constraint definition or usage whose body evaluates a Boolean expression.",
            Some("`constraint def Limit { in actual : Real; actual <= 10 }`"),
        ),
        "exhibit" => (
            "References or declares a state usage exhibited by a containing structure or behavior.",
            Some("`exhibit state lifecycle : Lifecycle;`"),
        ),
        "enum" => (
            "Enumeration definition or usage. Enumerated values are variant memberships of the definition.",
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
            "Event occurrence usage referring to an occurrence that happens in a temporal context.",
            Some("`event occurrence detected;`"),
        ),
        "snapshot" => (
            "Portion usage representing a zero-duration time slice of an occurrence.",
            Some("`snapshot atStartup;`"),
        ),
        "timeslice" => (
            "Portion usage representing a temporally bounded portion of an occurrence.",
            Some("`timeslice charging;`"),
        ),
        "calc" => (
            "Calculation definition/usage: computes a return value from its parameters.",
            Some("`calc def Identity { in x : Real; return : Real = x; }`"),
        ),
        "case" => (
            "General case definition or usage, the common basis for analysis, verification, and use cases.",
            Some("`case def Scenario;` or `case scenario : Scenario;`"),
        ),
        "analysis" => (
            "Analysis case definition or usage for evaluating an objective about a subject.",
            Some("`analysis def PerformanceAnalysis;`"),
        ),
        "verification" => (
            "Verification case definition/usage: verifies that a requirement is satisfied.",
            Some("`verification def RequirementTest;`"),
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
            "View definition or usage selecting model elements to expose; an optional rendering determines presentation.",
            Some("`view architecture { expose system::**; }`"),
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
            "View member selecting a rendering usage for the view.",
            Some("`render rendering diagram : DiagramRendering;`"),
        ),
        "expose" => (
            "Imports selected memberships or namespaces into the content exposed by a view.",
            Some("`expose Pkg::*;`"),
        ),
        "metadata" => (
            "Metadata definition or usage for annotating model elements; semantic metadata can additionally imply specialization.",
            Some("`metadata def Approval;` or `@Approval;`"),
        ),
        "meta" => (
            "Meta-cast operator that casts a type element to its reflective metadata definition or metaclass value.",
            Some("`element meta SysML::Usage`"),
        ),
        "concern" => (
            "Concern definition or usage describing a stakeholder issue as a specialized requirement.",
            Some("`concern name;`"),
        ),
        "stakeholder" => (
            "Stakeholder parameter of a requirement, concern, case, or viewpoint.",
            Some("`stakeholder name : Type;`"),
        ),
        "objective" => (
            "Objective requirement usage stating what a case sets out to achieve.",
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
            Some("`include use case includedCase;`"),
        ),
        "frame" => (
            "Requirement frame concern reference.",
            Some("`frame concern maintainability;`"),
        ),
        "filter" => (
            "Package element filter selecting elements that satisfy an expression.",
            Some("`filter @SysML::PartUsage;`"),
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
            Some("`private import Pkg::*;` or `public import Pkg::member;`"),
        ),
        "all" => (
            "Modifier on `import` that also imports otherwise-private members.",
            Some("`import all Pkg::*;`"),
        ),
        "flow" => (
            "Flow: item transfer between features over time.",
            Some("`flow source to target;` or `flow of Payload from source to target;`"),
        ),
        "message" => (
            "Message: a flow of a payload that triggers a behavior at its target.",
            Some("`message command of Command from sender to receiver;`"),
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
            "Introduces the port used by an accept or send action.",
            Some("`accept signal via inputPort;`"),
        ),
        "send" => (
            "Sends a payload to a target, optionally via a port.",
            Some("`send payload via outputPort to target;`"),
        ),
        "accept" => (
            "Accepts an incoming payload/event, optionally via a specific port.",
            Some("`accept payload via port;`"),
        ),
        "perform" => (
            "References or declares an action usage performed by a structure or behavior.",
            Some("`perform action process : Process;`"),
        ),
        "fork" => (
            "Control node that splits execution into concurrent flows.",
            Some("`fork split;`"),
        ),
        "join" => (
            "Control node that synchronizes concurrent flows.",
            Some("`join synchronize;`"),
        ),
        "merge" => (
            "Control node that combines alternative incoming flows.",
            Some("`merge alternatives;`"),
        ),
        "decide" => (
            "Control node that branches execution based on conditions.",
            Some("`decide choice;`"),
        ),
        "if" => (
            "Introduces a guard or a conditional action branch.",
            Some("`if condition { action yes; } else { action no; }`"),
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
            "Introduces a pre-test loop that repeats an action body while a condition holds.",
            Some("`while condition { action body; }`"),
        ),
        "loop" => (
            "Introduces an action loop whose optional `until` condition is checked after the body.",
            Some("`loop { action body; } until condition;`"),
        ),
        "for" => (
            "Loop that iterates over a collection.",
            Some("`for element in collection { action body; }`"),
        ),
        "until" => (
            "Loop-termination condition, checked after the body.",
            Some("`loop { action body; } until condition;`"),
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
            "Declares a return parameter of a calculation or case.",
            Some("`return name : Type;`"),
        ),
        "assert" => (
            "Introduces a constraint assertion or requirement-satisfaction assertion; `not` negates it.",
            Some("`assert constraint { expression }` or `assert satisfy requirement req by subject;`"),
        ),
        "assume" => (
            "Assumption constraint usage: a condition taken as given rather than checked.",
            Some("`assume constraint { expr }`"),
        ),
        "satisfy" => (
            "Requirement-satisfaction assertion relating a requirement usage to its subject.",
            Some("`assert satisfy requirement req by subject;`"),
        ),
        "require" => (
            "Requirement's evaluable condition.",
            Some("`require constraint { expr }`"),
        ),
        "crosses" => (
            "Textual form of cross-subsetting (`=>`), used to relate an end feature across a connector context.",
            Some("`end endpoint crosses otherEnd::connectedFeature;`"),
        ),
        "specializes" => (
            "Textual form of definition specialization (`:>`), making a definition more specific than another.",
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
            Some("`constant attribute name : Type = value;`"),
        ),
        "derived" => (
            "Feature modifier indicating the feature's value is computed rather than stored.",
            Some("`derived attribute name : Type = expression;`"),
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
            Some("`attribute name : Type default = 0;`"),
        ),
        "end" => (
            "Marks a feature as an end (connection point) of a connector-like definition/usage.",
            Some("`end producer : Type;`"),
        ),
        "parallel" => (
            "Marks a state as parallel: its substates execute concurrently with no transitions between them.",
            Some("`state Name parallel { ... }`"),
        ),
        "as" => (
            "Type-cast operator in a classification expression.",
            Some("`expression as Type`"),
        ),
        "istype" => (
            "Classification test for whether a value is directly classified by the given type.",
            Some("`x istype Type`"),
        ),
        "hastype" => (
            "Classification test for whether a value conforms to the given type, including specialization.",
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
            Some("`flow of Payload from source to target;`"),
        ),
        "to" => (
            "Introduces the target endpoint of a `flow`, `connect`, `allocate`, or `dependency`.",
            Some("`flow a to b;`"),
        ),
        "from" => (
            "Introduces the source of a flow/message or the client side of a dependency.",
            Some("`flow of Payload from a to b;` or `dependency from a to b;`"),
        ),
        "by" => (
            "Introduces the subject of a `satisfy`, or pairs with `defined`/`typed` (`defined by`, `typed by`).",
            Some("`assert satisfy req by subject;`"),
        ),
        "at" => (
            "Introduces an absolute-time trigger expression for an accept action.",
            Some("`accept at timeExpression;`"),
        ),
        "after" => (
            "Introduces a relative-time trigger expression for an accept action.",
            Some("`accept after durationExpression;`"),
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
            Some("`rep language \"OCL\" /* representation */`"),
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
    Some(KeywordHelp {
        description,
        syntax,
    })
}

/// Returns Markdown string for keyword hover (bold keyword, description, optional syntax hint).
/// Covers every word in [`RESERVED_KEYWORDS`] so hover never comes up empty for a normative
/// SysML v2 keyword. `None` is only returned for words that are not reserved.
pub fn keyword_hover_markdown(keyword: &str) -> Option<String> {
    let help = keyword_help(keyword)?;
    let mut md = format!("**{}**\n\n{}", keyword, help.description);
    if let Some(syn) = help.syntax {
        md.push_str(&format!("\n\nSyntax: {}", syn));
    }
    md.push_str("\n\n*See SysML v2 specification for full syntax.*");
    Some(md)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_keywords_match_omg_sysml_v2_0_clause_8_2_2_1_2() {
        let normative: Vec<&str> = OMG_SYSML_V2_0_RESERVED_KEYWORDS
            .split_whitespace()
            .collect();
        assert_eq!(RESERVED_KEYWORDS, normative.as_slice());
    }

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

    #[test]
    fn hover_markdown_is_built_from_the_structured_keyword_help() {
        for keyword in RESERVED_KEYWORDS {
            let help = keyword_help(keyword).expect("structured keyword help");
            let markdown = keyword_hover_markdown(keyword).expect("keyword hover markdown");
            assert!(markdown.contains(help.description), "{keyword}: {markdown}");
            if let Some(syntax) = help.syntax {
                assert!(markdown.contains(syntax), "{keyword}: {markdown}");
            }
        }
    }

    #[test]
    fn ordinary_identifiers_are_not_treated_as_sysml_reserved_keywords() {
        for identifier in ["value", "provides", "requires"] {
            assert!(!is_reserved_keyword(identifier), "{identifier}");
            assert!(keyword_hover_markdown(identifier).is_none(), "{identifier}");
        }
    }
}
